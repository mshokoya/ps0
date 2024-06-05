use tauri::{AppHandle, Manager};
use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account, domain::types::Domain, index::DB, metadata::types::Metadata, records::types::Record
        },
};



#[tauri::command]
pub async fn delete_accounts(ctx: AppHandle, args: Vec<String>) -> R<()> {
  for id in args.iter() {
    match ctx.state::<DB>().delete_one::<Account>("account", id).await {
      Ok(_) => {},
      Err(_) => return R::fail_none(None)
    }
  }
  R::ok_none()
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
pub async fn delete_domains(ctx: AppHandle, args: Vec<String>) -> R<()> {
  for id in args.iter() {
    match ctx.state::<DB>().delete_one::<Domain>("domain", id).await {
      Ok(_) => {},
      Err(_) => return R::fail_none(None)
    }
  }
  R::ok_none()
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