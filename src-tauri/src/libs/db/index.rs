use super::accounts::types::Account;
use super::domain::types::Domain;
use super::entity::{Entity, EntityTrait};
use super::metadata::types::Metadata;
use super::proxy::types::Proxy;
use super::records::types::Record;
use anyhow::Result;
use polodb_core::bson::{doc, from_document, to_document};
use polodb_core::results::{DeleteResult, UpdateResult};
use polodb_core::Collection;
use polodb_core::{bson::Document, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct DB {
    pub db: Arc<Mutex<Database>>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(Database::open_file("test-polo.db").unwrap())),
        }
    }

    pub fn init(&self) {
        let db = self.db.lock().unwrap();
        db.collection::<Account>("account");
        db.collection::<Domain>("domain");
        db.collection::<Record>("records");
        db.collection::<Metadata>("metadata");
        db.collection::<Proxy>("proxy");
    }

    fn get_collection(&self, entity_name: &'static str) -> Collection<Document> {
        self.db.lock().unwrap().collection::<Document>(entity_name)
    }

    pub fn insert_one(&self, entity: Entity, doc: Document) -> Result<String> {
        let collection = self.get_collection(entity.name());
        let result = collection.insert_one(doc)?;
        Ok(result.inserted_id.to_string())
    }

    pub fn find<T: DeserializeOwned>(
        &self,
        entity: Entity,
        filter: Option<Document>,
    ) -> Result<Vec<T>> {
        Ok(self
            .get_collection(entity.name())
            .find(filter)?
            .map(|entity| from_document::<T>(entity.unwrap()).unwrap())
            .collect::<Vec<T>>())
    }

    // pub fn find2<T: DeserializeOwned>(
    //     &self,
    //     entity: Entity,
    //     filter: Option<Document>,
    // ) -> Result<Vec<Document>> {
    //     let mut res = vec![];
    //     self
    //         .get_collection(entity.name())
    //         .find(filter)?
    //         .for_each(|r| res.push(r.unwrap()));
    //     Ok(res)
    // }

    // pub fn find2<T: DeserializeOwned>(
    //     &self,
    //     entity: Entity,
    //     filter: Option<Document>,
    // ) -> Result<Vec<T>> {
    //     Ok(self
    //         .get_collection(entity.name())
    //         .find(filter)?
    //         .map(|entity| {
    //             let mut doc = entity.unwrap();
    //             doc.insert("_id", doc.get("_id").unwrap()["jhgh"]);
    //             let doc = from_document::<T>(doc).unwrap();
    //         })
    //         .collect::<Vec<T>>())
    // }

    pub fn find_one<T: DeserializeOwned>(&self, entity: Entity, filter: Option<Document>) -> Option<T> {
        match self.get_collection(entity.name()).find_one(filter.unwrap()).unwrap() {
            Some(doc) => Some(from_document(doc).unwrap()),
            None => None,
        }
    }

    pub fn update_one(&self, entity: Entity, filter: Document, update: Document) -> Result<UpdateResult> {
        let collection = self.get_collection(entity.name());
        Ok(collection.update_one(filter, update)?)
    }

    pub fn delete<T: DeserializeOwned>(
        &self,
        entity: Entity,
        filter: Option<Document>,
    ) -> Result<DeleteResult> {
        Ok(self
            .get_collection(entity.name())
            .delete_many(filter.unwrap_or(doc! {}))?)
    }

    pub fn delete_one(&self, entity: Entity, filter: Document) -> Result<DeleteResult> {
        let collection = self.get_collection(entity.name());
        Ok(collection.delete_one(filter)?)
    }
}