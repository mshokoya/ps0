use anyhow::{Ok, Result};
use async_std::{sync::Mutex, task::block_on};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::local::{Db, File}, sql::Value, Surreal};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct DB(pub Arc<Mutex<Surreal<Db>>>);

impl DB {
    pub fn new() -> Result<Self> {
        let db = block_on::<_,Result<Surreal<Db>>>(async {
            let db  = Surreal::new::<File>("file://forwardscrape.db").await?;
            db.use_ns("forward_ns").use_db("forward_db").await?;
            Ok(db)
        })?;
        Ok(Self(Arc::new(Mutex::new(db))))
    }

    pub async fn insert_one<T: Serialize + DeserializeOwned + Debug>(&self, table: &str, id:&str, content: Value) -> Result<Option<Vec<T>>> {
        let entity: Option<Vec<T>> = self.0.lock()
            .await
            .query("CREATE $th SET $data")
            .bind(("th", format!("{table}:{id}")))
            .bind(("data", content))
            .await?
            .take(0)?;
        println!("INSERT ONE");
        println!("{entity:?}");
        Ok(entity)
    }

    pub async fn select_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock().await.query(format!("SELECT * FROM {} WHERE id={}", table, id)).await?.take(0)?;
        println!("SELECT ONE");
        println!("{entity:?}");
        Ok(entity)
    }

    pub async fn select_all<T: DeserializeOwned + Debug>(&self, table: &str) -> Result<Option<Vec<T>>> {
        let entity: Option<Vec<T>> = self.0.lock().await.query(format!("SELECT * FROM {}", table)).await?.take(0)?;
        println!("SELECT ALL");
        println!("{entity:?}");
        Ok(entity)
    }

    pub async fn update_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str, update: Value) -> Result<Option<Vec<T>>> {
        let entity:Option<Vec<T>> = self.0.lock().await
            .query("UPDATE $filter MERGE $data RETURN *")
            .bind(("filter", format!("{table}:{id}")))
            .bind(("data", update))
            .await?
            .take(0)?;
        println!("UPDATE ONE");
        println!("{entity:?}");
        Ok(entity)
    }

    pub async fn delete_one<T: DeserializeOwned + Debug>(&self, table: &str, id: &str) -> Result<Option<T>> {
        let entity: Option<T> = self.0.lock().await.delete((table, id)).await?;
        println!("DELETE ONE");
        println!("{entity:?}");
        Ok(entity)
    }
}