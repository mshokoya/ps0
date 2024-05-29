use std::collections::HashMap;
use serde::Deserialize;

use crate::libs::{db::metadata::types::{Accounts, Metadata}, taskqueue::types::TQTimeout};


const MAX_LEADS_ON_PAGE: u8 = 25;

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
