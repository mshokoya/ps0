use anyhow::{Context, Result};
use serde_json::json;
use super::types::{CreateDomain, VerifyDomain};

// (FIX) correct response data
pub struct Forwarder(pub String);

impl Forwarder {

  pub async fn add_domain(&self, domain: &str) -> Result<CreateDomain> {
    reqwest::Client::new()
    .post("https://api.forwardemail.net/v1/domains")
    .bearer_auth(self.0.clone())
    .json(&json!({
      "catchall":"auth@email.com", //()FIX use proper domain
      "domain": domain
    }))
    .send()
    .await?
    .json::<CreateDomain>()
    .await
    .context("Failed to create domain")
  }

  pub async fn verify_domain(&self, domain: &str) -> Result<VerifyDomain> {
    match reqwest::Client::new().get("https://api.forwardemail.net/v1/domains/hash.fyi/verify-records")
      .bearer_auth(self.0.clone())
      .send()
      .await {
        Ok(res) => {
          Ok(VerifyDomain {
            has_mx_record: true,
            has_txt_record: true,
            // message: res.text()
          })
        },
        Err(_) => {
          Ok(VerifyDomain {
            has_mx_record: false,
            has_txt_record: false,
          })
        }
      }
  }

  pub async fn delete_domain(&self, domain: &str) -> Result<()> {
    match reqwest::Client::new().delete(format!("https://api.forwardemail.net/v1/domains/{}", domain))
      .bearer_auth(self.0.clone())
      .send()
      .await {
        Ok(res) => Ok(()),
        Err(_) => Ok(())
      }
  }

  pub async fn get_domain(&self, domain: &str) -> Result<()> {
    match reqwest::Client::new().get(format!("https://api.forwardemail.net/v1/domains/{}", domain))
      .bearer_auth(self.0.clone())
      .send()
      .await {
        Ok(res) => Ok(()),
        Err(_) => Ok(())
      }
  }
}