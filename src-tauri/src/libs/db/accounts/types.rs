use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub _id: String,
    pub domain: String, // enum Domain
    pub trial_time: Option<String>,
    pub suspended: bool,
    pub login_type: String, // enum
    pub verified: String, // yes, no, confirm
    pub email: String,
    pub password: String,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<String>,
    pub renewal_start_date: Option<String>,
    pub renewal_end_date: Option<String>,
    pub last_used: Option<String>,
    pub cookies: Option<Cookies>,
    pub history: Vec<History>,
    pub total_scraped_recently: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
    pub total_page_scrape: u16,
    pub scrape_time: String,
    pub list_name: String,
    pub scrape_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies(String);

impl Into<Vec<CookieParam>> for Cookies {
    fn into(self: Self) -> Vec<CookieParam> {
        serde_json::from_str::<Vec<CookieParam>>(&self.0).unwrap()
        // let fields: Vec<CookieParse> = serde_json::from_str(&self.0).unwrap();
        // fields
        //     .iter()
        //     .map(|c| CookieParam::new(c.key.to_owned(), c.value.to_owned()))
        //     .collect::<Vec<CookieParam>>()
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CookieParse {
    pub key: String,
    pub value: String,
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