use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use polodb_core::bson::{
    oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string, to_document, Document
};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
use crate::libs::db::entity::EntityTrait;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {

    pub _id: String,
    pub domain: String, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: bool,
    pub login_type: String, // enum
    pub verified: String, // yes, no, confirm
    pub email: String,
    pub password: String,
    pub proxy: Option<String>,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<String>,
    pub renewal_start_date: Option<String>,
    pub renewal_end_date: Option<String>,
    pub last_used: Option<u64>,
    pub cookies: Option<Cookies>,
    pub history: Vec<History>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub total_page_scrape: Option<u8>,
    pub scrape_time: Option<u64>,
    pub list_name: Option<String>,
    pub scrape_id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies(String);

impl Into<Vec<CookieParam>> for Cookies {
    fn into(self: Self) -> Vec<CookieParam> {
        let fields: Vec<CookieParse> = serde_json::from_str(&self.0).unwrap();
        fields
            .iter()
            .map(|c| CookieParam::new(c.key.to_owned(), c.value.to_owned()))
            .collect::<Vec<CookieParam>>()
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CookieParse {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountArg {
    pub _id: Option<String>,
    pub domain: Option<String>, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: Option<bool>,
    pub login_type: Option<String>, // enum
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<String>,
    pub renewal_start_date: Option<String>,
    pub renewal_end_date: Option<String>,
    pub trial_days_left: Option<String>,
    pub last_used: Option<u64>,
    pub cookies: Option<Cookies>,
    pub history: Option<Vec<History>>,
}