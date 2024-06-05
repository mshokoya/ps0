use tauri::{AppHandle, Manager};
use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account, domain::types::Domain, index::DB, metadata::types::Metadata, records::types::Record
        },
};

#[tauri::command]
pub async fn get_accounts(ctx: AppHandle) -> R<Vec<Account>> {
  match ctx.state::<DB>().select_all::<Account>("account").await {
    Ok(docs) => R::ok_data(docs.unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub async fn get_metadatas(ctx: AppHandle) -> R<Vec<Metadata>> {
  match ctx.state::<DB>().select_all::<Metadata>("metadata").await {
    Ok(docs) => R::ok_data(docs.unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub async fn get_domains(ctx: AppHandle) -> R<Vec<Domain>> {
  match ctx.state::<DB>().select_all::<Domain>("domain").await {
    Ok(docs) => R::ok_data(docs.unwrap()),
    Err(_) => R::ok_none()
  }
}

#[tauri::command]
pub async fn get_records(ctx: AppHandle) -> R<Vec<Record>> {
  match ctx.state::<DB>().select_all::<Record>("record").await {
    Ok(docs) => R::ok_data(docs.unwrap()),
    Err(_) => R::ok_none()
  }
}

