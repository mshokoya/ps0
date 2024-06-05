use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApolloCheckArgs {
    pub account_id: String,
}
