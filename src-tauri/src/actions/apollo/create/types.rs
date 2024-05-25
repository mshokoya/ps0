use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApolloCreateArgs {
  pub domain_id: String,
  pub domain: String
}