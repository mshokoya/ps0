use polodb_core::bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApolloCheckArgs {
    pub account_id: String,
}
