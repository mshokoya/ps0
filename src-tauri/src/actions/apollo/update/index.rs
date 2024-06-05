use polodb_core::bson::{doc, to_bson, to_document, Document};
use serde::Deserialize;
use serde_json::{from_value, to_value, Value};
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::{Response as R, Response2 as R2},
    libs::db::{
            accounts::types::Account, entity::Entity, index::DB
        },
};

#[derive(Debug, Deserialize)]
struct UpdateFields {
  pub email: Option<String>,
  pub password: Option<String>
}

#[derive(Debug, Deserialize)]
struct UpdateAccArg {
    pub account_id: String,
    pub fields: UpdateFields
}

#[tauri::command]
pub fn update_account(ctx: AppHandle, args: Document) -> R2<Document> {
  // let args: UpdateAccArg = from_value(&args).unwrap();
  // let update = to_document(&args.fields).unwrap();

  println!("{:?}", args);

  let id = doc!{"_id": args.get("account_id").unwrap()};
  let update = args.get("fields").unwrap();

  let db = ctx.state::<DB>();

  match db.update_one(Entity::Account, id.clone(), doc! { "$set":  update}) {
    Ok(_) =>{
      let acc = db.find_one::<Account>(Entity::Account,Some(id)).unwrap();
    R2::ok_data(to_document(&acc).unwrap())
    },
    Err(_) => R2::fail_none()
  }
}

#[tauri::command]
pub fn update_metadata(ctx: AppHandle, filter: Document, update: Document) -> R {
  match ctx.state::<DB>().update_one(Entity::Metadata, filter, doc! { "$set": to_bson(&update).unwrap() }) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub fn update_domain(ctx: AppHandle, filter: Document, update: Document) -> R {
  match ctx.state::<DB>().update_one(Entity::Domain, filter, doc! { "$set": to_bson(&update).unwrap() }) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub fn update_record(ctx: AppHandle, filter: Document, update: Document) -> R {
  match ctx.state::<DB>().update_one(Entity::Record, filter, doc! { "$set": to_bson(&update).unwrap() }) {
    Ok(docs) => R::ok_data(to_value(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}