use serde_json::Value;
use tauri::{AppHandle, Manager};
use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account, domain::types::Domain, index::DB, metadata::types::Metadata, records::types::{Record, RecordArg}
        },
};

#[tauri::command]
pub async fn get_accounts(ctx: AppHandle) -> R<Vec<Value>> {
  match ctx.state::<DB>().select_all::<Value>("account").await {
    Ok(docs) => R::ok_data(docs),
    Err(e) => {
      println!("GET_ACCOUNTS FAIL");
      println!("{e:?}");
      R::ok_none()
    }
  }
}

// #[tauri::command]
// pub async fn get_account(ctx: AppHandle) -> R<Account> {
//   match ctx.state::<DB>().select_one::<Account>("account", "19imojfg9ywmo9bgxcdd").await {
//     Ok(docs) => {
//       if docs.is_none() {
//         return R::fail_none(Some("Failed to find account"))
//       }
//       R::ok_data(docs.unwrap())
//     },
//     Err(e) =>  R::ok_none()
//   }
// }

#[tauri::command]
pub async fn get_metadatas(ctx: AppHandle) -> R<Vec<Value>> {
  match ctx.state::<DB>().select_all::<Value>("metadata").await {
    Ok(docs) => {
      R::ok_data(docs)
    },
    Err(e) => {
      println!("GET_ACCOUNTS FAIL");
      println!("{e:?}");
      R::ok_none()
    }
  }
}

#[tauri::command]
pub async fn get_domains(ctx: AppHandle) -> R<Vec<Domain>> {
  match ctx.state::<DB>().select_all::<Domain>("domain").await {
    Ok(docs) => {
      R::ok_data(docs)
    },
    Err(e) => {
      println!("GET_ACCOUNTS FAIL");
      println!("{e:?}");
      R::ok_none()
    }
  }
}

#[tauri::command]
pub async fn get_records(ctx: AppHandle) -> R<Vec<Value>> {
  match ctx.state::<DB>().select_all::<Value>("record").await {
    Ok(docs) => {
        R::ok_data(docs)
    },
    Err(e) => {
      println!("GET_ACCOUNTS FAIL");
      println!("{e:?}");
      R::ok_none()
    }
  }
}

