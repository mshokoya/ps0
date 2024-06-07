use serde::Deserialize;
use serde_json::{from_value, Value};
use tauri::{AppHandle, Manager};
use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account, domain::types::Domain, index::DB, metadata::types::Metadata, records::types::Record
        },
};

#[derive(Deserialize)]
struct DeleteOneArg {
  pub id: String
}

#[tauri::command]
pub async fn delete_account(ctx: AppHandle, args: Value) -> R<()> {
  let arg: DeleteOneArg = match from_value(args) {
    Ok(args) => args,
    Err(_) => return R::fail_none(Some("Failed to parse id"))
  };

  match ctx.state::<DB>().delete_one::<Account>("account", &arg.id).await {
    Ok(_) => R::ok_none(),
    Err(_) => R::fail_none(Some("Failed to remove domain"))
  }
}

#[tauri::command]
pub async fn delete_metadatas(ctx: AppHandle, args: Vec<String>) -> R<()> {
  for id in args.iter() {
    match ctx.state::<DB>().delete_one::<Metadata>("metadata", id).await {
      Ok(_) => {},
      Err(_) => return R::fail_none(None)
    }
  }
  R::ok_none()
}

#[tauri::command]
pub async fn delete_domain(ctx: AppHandle, args: Value) -> R<()> {
  let arg: DeleteOneArg = match from_value(args) {
    Ok(args) => args,
    Err(_) => return R::fail_none(Some("Failed to parse id"))
  };

  match ctx.state::<DB>().delete_one::<Domain>("domain", &arg.id).await {
    Ok(_) => R::ok_none(),
    Err(_) => R::fail_none(Some("Failed to remove domain"))
  }
}

#[tauri::command]
pub async fn delete_records(ctx: AppHandle, args: Vec<String>) -> R<()> {
  for id in args.iter() {
    match ctx.state::<DB>().delete_one::<Record>("record", id).await {
      Ok(_) => {},
      Err(_) => return R::fail_none(None)
    }
  }
  R::ok_none()
}