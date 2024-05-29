use std::collections::HashMap;
use serde::Deserialize;
use anyhow::{anyhow, Result};
use polodb_core::bson::{doc, to_bson, Uuid};
use serde_json::{from_value, json, Value};
use tauri::{AppHandle, Manager, State};
use crate::actions::apollo::lib::index::log_into_apollo;
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
  pub max_leads: u64,
  pub task_id: String,
  pub params: HashMap<String, String>,
}

#[derive(Deserialize)]
struct ScrapeActionArgs {
  pub url: String,
  pub chunk: [u64; 2],
  pub account_id: String,
  pub metadata: String,
  pub max_leads: u64,
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
        "max_leads": &fmt_args.max_leads
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

    let mut url = set_range_in_apollo_url(args.url, args.chunk);
    url = set_page_in_apollo_url(args.url, 1);

    let apollo_max_page = if account.domain.contains("gmail") || 
    account.domain.contains("hotmail") ||
    account.domain.contains("outlook") 
    { 3 } else { 5 };

    let prev_name = PreviousLead {
      name: "".to_string(),
    };

    

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