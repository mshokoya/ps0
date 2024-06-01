use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateDomain {
  pub has_mx_record: bool,
  pub has_txt_record: bool,
  // pub id: String
}

#[derive(Deserialize, Serialize)]
pub struct VerifyDomain {
  pub has_mx_record: bool,
  pub has_txt_record: bool,
  // pub id: String
}

#[derive(Deserialize)]
pub struct DeleteDomain {
  pub retention_days: usize,
  pub has_regex: bool,
  pub has_catchall: bool,
  pub has_adult_content_protection: bool,
  pub has_phishing_protection: bool,
  pub has_executable_protection: bool,
  pub has_virus_protection: bool,
  pub is_catchall_regex_disabled: bool,
  pub plan: String,
  pub max_recipients_per_alias: usize,
  pub smtp_port: String,
  // members: [ { user: [Object], group: 'admin' } ], ?
  pub name: String,
  pub has_mx_record: bool,
  pub has_txt_record: bool,
  pub verification_record: String,
  pub has_recipient_verification: bool,
  pub has_custom_verification: bool,
  pub id: String,
  pub object: String,
  pub locale: String,
  // created_at: Date
  // updated_at: Date
  pub link: String,
}