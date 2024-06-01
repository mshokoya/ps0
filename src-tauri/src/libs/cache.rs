use std::collections::HashMap;

use anyhow::Result;
use async_std::sync::Mutex;


pub struct ApolloCache(pub Mutex<HashMap<String, Vec<String>>>);

impl ApolloCache {
  pub async fn get(&self, meta_id: &str) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn delete(&self, meta_id: &str) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn delete_meta(&self, meta_id: &str) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn get_meta(&self, meta_id: &str) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn add_accounts(&self, meta_id: &str, account_ids: &Vec<String>) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn remove_account(&self, meta_id: &str, account_id: &str) -> Option<&Vec<String>> {
    self.0.lock().await.get(meta_id);
    todo!()
  }

  pub async fn get_all_meta_ids(&self) -> Option<&Vec<String>> {
    self.0.lock().await;
    todo!()
  }

  pub async fn get_all_account_ids(&self) -> Option<&Vec<String>> {
    self.0.lock().await;
    todo!()
  }
}