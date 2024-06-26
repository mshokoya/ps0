use anyhow::{Ok, Result};
use async_std::{sync::Mutex, task::block_on};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::local::{Db, File}, Surreal};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct DB(pub Arc<Mutex<Surreal<Db>>>);

impl DB {
    pub fn new() -> Result<Self> {
        let db = block_on::<_,Result<Surreal<Db>>>(async {
            let db  = Surreal::new::<File>("file://Applications/forwardscraper/forward.db").await?;
            db.use_ns("forwardns").use_db("forwarddb").await?;
            Ok(db)
        })?;
        Ok(Self(Arc::new(Mutex::new(db))))
    }

    pub async fn insert_one<T: Serialize + DeserializeOwned + Debug>(&self, table: &str, id:&str, content: impl Serialize) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock()
            .await
            .query(format!(r#"CREATE {table}:{id} CONTENT $data"#))
            .bind(("data", content))
            .await?
            .take(0)?;
        Ok(entity)
    }

    pub async fn select_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock().await.query(format!(r#"SELECT * FROM {} WHERE _id="{}""#, table, id)).await?.take(0)?;
        Ok(entity)
    }

    pub async fn select_all<T: DeserializeOwned + Debug>(&self, table: &str) -> Result<Vec<T>> {
        let entity: Vec<T> = self.0.lock().await.query(format!("SELECT * FROM {}", table)).await?.take(0)?;
        Ok(entity)
    }

    pub async fn update_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str, update: impl Serialize) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock().await
            .query(format!(r#"UPDATE {table}:{id} MERGE $data RETURN AFTER"#))
            .bind(("data", update))
            .await?
            .take(0)?;

        Ok(entity)
    }

    pub async fn delete_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock().await.delete((table, id)).await?;
        println!("DELETE ONE");
        println!("{entity:?}");
        Ok(entity)
    }
}