use polodb_core::bson::{oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use crate::libs::db::entity::EntityTrait;

// https://serde.rs/field-attrs.html

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub _id: String,
    pub scrape_id: String,
    pub url: String,
    pub data: RecordData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordData {
    pub name: String,
    pub firstname: String,
    pub lastname: String,
    pub linkedin: String,
    pub title: String,
    pub email_1: Option<String>,
    pub email_2: Option<String>,
    pub email_3: Option<String>,
    pub company_name: String,
    pub company_website: String,
    pub comapny_linkedin: String,
    pub company_twitter: String,
    pub company_facebook: String,
    pub email: String,
    pub is_verified: bool,
    pub company_location: String,
    pub employees: String,
    pub phone: String,
    pub industry: String,
    pub keywords: Vec<String>,
}

pub struct RecordArg {
    pub _id: Option<String>,
    pub scrape_id: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub data: RecordDataArg,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordDataArg {
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub linkedin: Option<String>,
    pub title: Option<String>,
    pub company_name: Option<String>,
    pub company_website: Option<String>,
    pub comapny_linkedin: Option<String>,
    pub company_twitter: Option<String>,
    pub company_facebook: Option<String>,
    pub email_1: Option<String>,
    pub email_2: Option<String>,
    pub email_3: Option<String>,
    pub is_verified: Option<bool>,
    pub company_location: Option<String>,
    pub employees: Option<String>,
    pub phone: Option<String>,
    pub industry: Option<String>,
    pub keywords: Option<Vec<String>>,
}