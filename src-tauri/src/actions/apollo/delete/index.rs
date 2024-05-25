use polodb_core::bson::{doc, to_bson};
use serde_json::{to_value, Value};
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account,
            entity::Entity,
            index::DB,
        },
};


#[tauri::command]
pub fn delete_accounts(ctx: AppHandle, args: Vec<Value>) -> R {
  let filter = if args.is_empty() {
    None
  } else {
    Some(doc! { "$or": to_bson(&args).unwrap() })
  };

  match ctx.state::<DB>().delete::<Account>(Entity::Account, filter) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}