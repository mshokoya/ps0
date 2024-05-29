use std::cmp;
use std::collections::HashMap;
use std::time::Duration;
use async_std::task::sleep;
use fake::faker::internet::en::Username;
use fake::Fake;
use serde::Deserialize;
use anyhow::{anyhow, Result};
use polodb_core::bson::{doc, to_bson, Uuid};
use serde_json::{from_value, json, Value};
use tauri::{AppHandle, Manager, State};
use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo};
use crate::actions::apollo::lib::util::time_ms;
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::Account;
use crate::libs::db::metadata::types::{Accounts, Metadata, Scrapes};
use crate::libs::taskqueue::index::TaskQueue;
use crate::libs::taskqueue::types::TQTimeout;
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

const MAX_LEADS_ON_PAGE: u8 = 25;

#[derive(Deserialize)]
struct ScrapeTaskArgs {
  pub name: String,
  pub meta_id: String,
  pub url: String,
  pub accounts: Vec<Accounts>,
  pub timeout: TQTimeout,
  pub max_leads_limit: u64,
  pub task_id: String,
  pub params: HashMap<String, String>,
}

#[derive(Deserialize)]
struct ScrapeActionArgs {
  pub url: String,
  pub chunk: [u64; 2],
  pub account_id: String,
  pub metadata: Metadata,
  pub max_leads_limit: u64,
}

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

    // let mut url = set_range_in_apollo_url(args.url, args.chunk).await;
    // url = set_page_in_apollo_url(args.url, 1).await;

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
      let list_name = Username().fake();
  
      // update_db_for_new_scrape(&ctx.task_id, &metadata, &account, &list_name, &scrape_id);
  
      // page.goto(url).await goToApolloSearchUrl

      let data = add_leads_to_list_and_scrape(&ctx, &num_leads_to_scrape, &list_name, prev_lead).await;

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


struct PreviousLead {
  name: String,
}

impl PreviousLead {
  fn get(&self) -> String {
    self.name
  }

  fn set(&self, name: String) {
    self.name = name;
  }
}