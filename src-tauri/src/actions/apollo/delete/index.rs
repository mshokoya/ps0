use polodb_core::bson::{doc, to_bson};
use serde_json::{to_value, Value};
use tauri::{AppHandle, Manager};

use crate::{
    actions::{apollo::lib::util::filter, controllers::Response as R},
    libs::db::{
            accounts::types::Account, domain::types::Domain, entity::Entity, index::DB, metadata::types::Metadata, records::types::Record
        },
};


#[tauri::command]
pub fn delete_accounts(ctx: AppHandle, args: Vec<Value>) -> R {
  let filter = filter(args);
  match ctx.state::<DB>().delete::<Account>(Entity::Account, filter) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub fn delete_metadatas(ctx: AppHandle, args: Vec<Value>) -> R {
  let filter = filter(args);
  match ctx.state::<DB>().delete::<Metadata>(Entity::Metadata, filter) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub fn delete_domains(ctx: AppHandle, args: Vec<Value>) -> R {
  let filter = filter(args);
  match ctx.state::<DB>().delete::<Domain>(Entity::Domain, filter) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub fn delete_records(ctx: AppHandle, args: Vec<Value>) -> R {
  let filter = filter(args);
  match ctx.state::<DB>().delete::<Record>(Entity::Record, filter) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}