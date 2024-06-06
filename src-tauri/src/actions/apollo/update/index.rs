use serde::Deserialize;
use serde_json::{from_value, Value as Serde_Value};
use surrealdb::sql::Value;
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account, domain::types::Domain, index::DB, metadata::types::Metadata, records::types::Record
        },
};



#[derive(Debug, Deserialize)]
pub struct UpdateArg {
    pub account_id: String,
    pub fields: Value
}

#[tauri::command]
pub async fn update_account(ctx: AppHandle, args: UpdateArg) -> R<Account> {
  // println!("UPDATE ONE 111");
  // println!("{args:#?}");
  // let p_args = match from_value::<UpdateArg>(args) {
  //   Ok(var) => var,
  //   Err(_) => return R::fail_none(Some("Failed to update account, could not get the data"))
  // };
  // println!("UPDATE ONE 222");
  // println!("{p_args:#?}");

  match ctx.state::<DB>()
    .update_one::<Account>(
      "account", 
      &args.account_id, 
      args.fields
    ).await {
    Ok(acc) => {
      println!("UPDATE ONE 333");
      println!("{acc:#?}");
      R::ok_data(acc.unwrap())
    },
    Err(e) => {
      println!("UPDATE ONE 444");
      println!("{:#?}", e);
      R::fail_none(None)
    }
  }
}

#[tauri::command]
pub async fn update_metadata(ctx: AppHandle, args: Serde_Value) -> R<Metadata> {
  let p_args = match from_value::<UpdateArg>(args) {
    Ok(var) => var,
    Err(_) => return R::fail_none(Some("Failed to update metadata, could not get the data"))
  };

  match ctx.state::<DB>()
    .update_one::<Metadata>(
      "metadata", 
      &p_args.account_id, 
      p_args.fields
    ).await {
    Ok(acc) => R::ok_data(acc.unwrap()),
    Err(_) => R::fail_none(None)
  }
}



#[tauri::command]
pub async fn update_domain(ctx: AppHandle, args: Serde_Value) -> R<Domain> {
  let p_args = match from_value::<UpdateArg>(args) {
    Ok(var) => var,
    Err(_) => return R::fail_none(Some("Failed to update domain, could not get the data"))
  };

  match ctx.state::<DB>()
    .update_one::<Domain>(
      "domain", 
      &p_args.account_id, 
      p_args.fields
    ).await {
    Ok(acc) => R::ok_data(acc.unwrap()),
    Err(_) => R::fail_none(None)
  }
}

#[tauri::command]
pub async fn update_record(ctx: AppHandle, args: Serde_Value) -> R<Record> {
  let p_args = match from_value::<UpdateArg>(args) {
    Ok(var) => var,
    Err(_) => return R::fail_none(Some("Failed to update record, could not get the data"))
  };

  match ctx.state::<DB>()
    .update_one::<Record>(
      "record", 
      &p_args.account_id, 
      p_args.fields
    ).await {
    Ok(acc) => R::ok_data(acc.unwrap()),
    Err(_) => R::fail_none(None)
  }
}