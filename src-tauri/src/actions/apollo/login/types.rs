use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApolloLoginArgs {
  pub account_id: String
}