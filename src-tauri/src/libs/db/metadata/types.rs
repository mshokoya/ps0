use std::collections::HashMap;

use polodb_core::bson::{oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
#[derive(Debug, Serialize)]
pub struct Metadata {
    _id: ObjectId,
    url: String,
    params: HashMap<String, Value>,
    name: String,
    scrapes: Vec<Scrapes>,
    accounts: Vec<Accounts>,
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
    account_id: ObjectId,
    range: [u32; 2],
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Scrapes {
    scrape_id: ObjectId,
    list_name: String,
    length: u8,
    data: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataArg {
    _id: Option<ObjectId>,
    url: Option<String>,
    params: Option<HashMap<String, Value>>,
    name: Option<String>,
    scrapes: Option<Vec<Scrapes>>,
    accounts: Option<Vec<Accounts>>,
}