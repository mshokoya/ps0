use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Domain {
    pub _id: String,
    pub domain: String,
    pub verified: bool,
    pub mx_records: bool,
    pub txt_records: bool,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainArg {
    pub _id: Option<String>,
    pub domain: Option<String>,
    verified: Option<bool>,
    mx_records: Option<bool>,
    txt_records: Option<bool>,
    message: Option<String>,
}