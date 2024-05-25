use polodb_core::bson::{oid::ObjectId, to_document, Document};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Proxy {
    _id: String,
    proxy: String,
    protocol: String,
    host: String,
    port: String,
}

pub struct ProxyArg {
    _id: Option<ObjectId>,
    proxy: Option<String>,
    protocol: Option<String>,
    host: Option<String>,
    port: Option<String>,
}