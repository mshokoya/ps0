use std::collections::HashMap;
use async_std::sync::Mutex;


pub struct ApolloCache(pub Mutex<HashMap<String, Vec<String>>>);

impl ApolloCache {
  pub fn new() -> Self {
    Self(Mutex::new(HashMap::new()))
  }

  pub async fn get(&self, meta_id: &str) -> Option<Vec<String>> {
    self.0.lock().await.get(meta_id).cloned()
  }

  pub async fn delete(&self, meta_id: &str) {
    self.0.lock().await.remove(meta_id);
  }

  pub async fn add_accounts(&self, meta_id: &str, account_ids: &mut Vec<String>) {
    self.0.lock().await.get_mut(meta_id).get_or_insert(&mut Vec::new()).append(account_ids);
  }

  pub async fn remove_account(&self, meta_id: &str, account_id: &str) {
    self.0.lock().await.get_mut(meta_id).get_or_insert(&mut Vec::new()).retain(|v| v != account_id);
  }

  pub async fn get_all_meta_ids(&self) -> Vec<String> {
    self.0.lock().await.keys().cloned().collect::<Vec<String>>()
  }

  pub async fn get_all_account_ids(&self) -> Vec<String> {
    let mut ids :Vec<String> = vec![];
    self.0.lock().await.iter_mut().for_each(|(_,v)| {
      ids.append(v.clone().as_mut());
    });
    ids
  }
}