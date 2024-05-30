use std::cmp::{self, max, min};
use std::thread::spawn;
use std::time::Duration;
use async_std::task::{block_on, sleep, spawn};
use chromiumoxide::Page;
use fake::faker::internet::en::Username;
use fake::Fake;
use serde::Deserialize;
use anyhow::{anyhow, Result};
use polodb_core::bson::{doc, to_bson, Uuid};
use serde_json::{from_value, json, Value};
use tauri::{AppHandle, Manager, State};
use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo};
use crate::actions::apollo::lib::util::{set_page_in_url, set_range_in_url, time_ms, wait_for_selector};
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::{Account, History};
use crate::libs::db::metadata;
use crate::libs::db::metadata::types::{Metadata, Scrapes};
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::{
        db::{
            entity::Entity,
            index::DB,
        },
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

use super::types::{PreviousLead, ScrapeActionArgs, ScrapeTaskArgs, MAX_LEADS_ON_PAGE};


#[tauri::command]
pub fn scrape_task(ctx: AppHandle, args: Value) -> R {
    // (FIX) should create meta_id in backend 
    let meta_id = match args.get("meta_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    let fmt_args: ScrapeTaskArgs = from_value(args.clone()).unwrap();
    let metadata = init_meta(&ctx.state::<DB>(), &fmt_args);

    args.get("accounts").iter().copied().for_each(|acc| {
      let argss = json!({
        "url": &fmt_args.url,
        "range": acc.get("range").as_ref().unwrap(),
        "account_id": acc.get("account_id").as_ref().unwrap(),
        "metadata": &metadata,
        "max_leads_limit": &fmt_args.max_leads_limit
      });

      ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new(),
        task_type: TaskType::ApolloScrape,
        task_group: TaskGroup::Apollo,
        message: "Scraping",
        metadata: meta_id.clone(),
        timeout: args.get("timeout").and_then(|v| from_value(v.clone()).ok()),
        args: Some(argss),
      });
    });

    R::ok_none()
}

pub async fn apollo_scrape(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    let args: ScrapeActionArgs = from_value(args.unwrap())?;
    ctx.page =  Some(unsafe { SCRAPER.incog().await? });
    let page = ctx.page.as_ref().unwrap();
    let db = ctx.handle.state::<DB>();
    
    let account = db.find_one::<Account>(
      Entity::Account,
      Some(doc! {"_id": &args.account_id})
    ).unwrap();

    log_into_apollo(&ctx, &account).await?;

    let mut url = set_range_in_url(args.url, args.chunk);
    url = set_page_in_url(args.url, 1);

    let apollo_max_page = if account.domain.contains("gmail") || 
    account.domain.contains("hotmail") ||
    account.domain.contains("outlook") { 3 } else { 5 };

    let prev_lead = PreviousLead {
      name: "".to_string(),
    };

    let old_credits = apollo_login_credits_info(&ctx).await?;

    while (args.max_leads_limit > 0) {
      let credits_left = old_credits.credits_limit - old_credits.credits_used;
      if credits_left <= 0 { return Ok(None) }

      let mut num_leads_to_scrape = cmp::min(args.max_leads_limit, credits_left.into());
      num_leads_to_scrape = cmp::min(num_leads_to_scrape, MAX_LEADS_ON_PAGE.into());
      if num_leads_to_scrape <= 0 { return Ok(None) }

      let scrape_id = Uuid::new().to_string();
      let list_name = Username().fake::<String>();
  
      update_db_for_new_scrape(&ctx, &mut args.metadata, &mut account, &list_name, &scrape_id);
  
      go_to_search_url(page, &url).await;

      let data = add_leads_to_list_and_scrape(
        &ctx, 
        &num_leads_to_scrape, 
        &list_name,
        &prev_lead
      ).await;

      if data.is_none() || data.unwrap().len() == 0 {
        return Ok(None)
      }

      sleep(Duration::from_secs(3)).await;

      let new_credits = apollo_login_credits_info(&ctx).await?;
      let cookies = get_browser_cookies(&ctx).await;
      let total_scraped = data.unwrap().len();
      account.total_scraped_recently += total_scraped;
      account.history.push(vec![total_scraped, time_ms(), &list_name, &scrape_id]);

      let save = save_scrape_to_db(
        &ctx,
        &account,
        &args.metadata,
        &new_credits,
        &cookies,
        &list_name,
        &args.chunk,
        &data.unwrap(),
      );

      let mut next_page: u8 = get_page_in_url(url) + 1;
      next_page = if next_page > apollo_max_page { 1 } else { next_page };
      url = set_page_in_url(&url, next_page);
      args.metadata = save.metadata;
      account = save.account;
      args.max_leads_limit = args.max_leads_limit - total_scraped;
      old_credits = new_credits;
    }



    Err(anyhow!("Failed to signup: please try again"))
}

fn init_meta(db: &State<DB>, args: &ScrapeTaskArgs) -> Metadata {
  match db.find_one::<Metadata>(
    Entity::Metadata,
    Some(doc! {"_id": &args.meta_id})
  ) {
    Some(meta) => meta,
    None => {
      let scrapes: Vec<Scrapes> = vec![];
      db.insert_one(
        Entity::Metadata,
        doc! {
          "_id": &args.meta_id,
          "url": &args.url,
          "params": to_bson(&args.params).unwrap(),
          "name": &args.name,
          "scrapes": to_bson(&scrapes).unwrap(),
          "accounts": to_bson(&args.accounts).unwrap(),
        }
      ).unwrap();
      db.find_one::<Metadata>(
        Entity::Metadata,
        Some(doc! {"_id": &args.meta_id})
      ).unwrap()
    }
  }
}

async fn update_db_for_new_scrape(ctx: &TaskActionCTX, metadata: &mut Metadata, account: &mut Account, list_name: &str, scrape_id: &str) -> Result<()> {
  let db_state = ctx.handle.state::<DB>();
  let db = db_state.db.lock().unwrap();

  let mut session = db.start_session()?;
  session.start_transaction(None)?;

  let metadata_collection = db.collection::<Metadata>(&Entity::Metadata.name());
  let account_collection = db.collection::<Account>(&Entity::Account.name());

  metadata.scrapes.push(Scrapes {
    scrape_id: scrape_id.to_string(), 
    list_name: list_name.to_string(), 
    length: 0, 
    date: time_ms()
  });

  metadata_collection.update_one_with_session(
    doc! {"_id": &metadata._id}, 
    doc! {"scrapes": to_bson(&metadata.scrapes)?}, 
    &mut session
  )?;

  account.history.push(History {
    total_page_scrape: None, 
    scrape_time: None, 
    list_name: Some(list_name.to_string()),
    scrape_id: Some(scrape_id.to_string())
  });

  account_collection.update_one_with_session(
    doc! {"_id": &account._id}, 
    doc! {"history": to_bson(&account.history)?},
    &mut session
  )?;

  session.commit_transaction()?;

  Ok(())
}

async fn go_to_search_url(page: &Page, url: &str) -> Result<()> {
  page.goto(url).await?;
  wait_for_selector(page, r#"[class="zp_RFed0"]"#, 10, 2).await?;
  Ok(())
}

async fn add_leads_to_list_and_scrape(ctx: &TaskActionCTX, num_leads_to_scrape: &u16, list_name: &str, prev_lead: &PreviousLead) -> Result<()> {
  let table_rows_selector = r#"[class="zp_RFed0"]"#;
  let checkbox_selector = r#"[class="zp_fwjCX"]"#;
  let add_to_list_input_selector = r#"[class="Select-input "]"#;
  let save_list_button_selector = r#"[class="zp-button zp_zUY3r"][type="submit"]"#;
  let sl_popup_selector = r#"[class="zp_lMRYw zp_yHIi8"]"#;
  let saved_list_table_row_selector = r#"[class="zp_cWbgJ"]"#;
  let pagination_info_selector = r#"[class="zp_VVYZh"]"#;

  let page = ctx.page.as_ref().unwrap();

  wait_for_selector(&page, pagination_info_selector, 10, 2).await?;

  let mut name = "".to_string();
  let mut should_continue = false;
  let mut counter: u8 = 0;
  while !should_continue && counter < 15 {
    name = get_first_table_row_name(&page).await?.unwrap();
    if name != prev_lead.get() { should_continue = true; }
    sleep(Duration::from_secs(2)).await;
    counter += 1;
  }

  prev_lead.set(name);

  let rows = page.find_elements(table_rows_selector).await?;
  let max_leads = min(*num_leads_to_scrape, rows.len() as u16);

  for row_idx in 0..max_leads {
    if let Some(el) = rows.get(row_idx as usize) {
      el.focus().await?.click().await?;
    }
  }

  let list_button = page.find_elements(r#"[class="zp-button zp_zUY3r zp_hLUWg zp_n9QPr zp_B5hnZ zp_MCSwB zp_ML2Jn"]"#).await?;
  if list_button.len() == 0 { return Err(anyhow!("failed to find list button")) }
  list_button[1].focus().await?.click().await?;




  todo!()
}

async fn get_first_table_row_name(page: &Page) -> Result<Option<String>> {
  let mut row = page.find_element(r#"[class="zp_BC5Bd"]"#).await?;
  row = row.find_element(r#"[class="zp_BC5Bd"]"#).await?;
  Ok(row.inner_text().await?)
}