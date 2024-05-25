use polodb_core::bson::{doc, Document, to_bson};
use serde_json::to_value;
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R,
    libs::db::{
            entity::Entity,
            index::DB,
        },
};


#[tauri::command]
pub fn update_account(ctx: AppHandle, filter: Document, update: Document) -> R {
  match ctx.state::<DB>().update_one(Entity::Account, filter, doc! { "$set": to_bson(&update).unwrap() }) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}