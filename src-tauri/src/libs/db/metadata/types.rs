use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Id;
// use ts_rs::TS;

// export type IMetaData = {
//   id: string
//   url: string
//   params: { [key: string]: string }
//   name: string
//   scrapes: { scrapeID: string; listName: string; length: number; date: number }[]
//   accounts: { accountID: string; range: [min: number, max: number] }[]
// }

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub _id: String,
    pub url: String,
    // pub params: HashMap<String, String>,
    pub name: String,
    pub scrapes: Vec<Scrapes>,
    pub accounts: Vec<Accounts>,
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accounts {
    pub account_id: String,
    pub chunk: [u32; 2],
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scrapes {
    pub scrape_id: String,
    pub list_name: String,
    pub length: u8,
    pub date: u128,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataArg {
    pub _id: Option<Id>,
    pub url: Option<String>,
    pub params: Option<HashMap<String, Value>>,
    pub name: Option<String>,
    pub scrapes: Option<Vec<Scrapes>>,
    pub accounts: Option<Vec<Accounts>>,
}