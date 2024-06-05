use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::{actions::controllers::Response as R, libs::forwarder::{index::Forwarder, types::{CreateDomain, VerifyDomain}}};


#[derive(Deserialize)]
pub struct Domain {
  domain: String
}

// pub async fn get_domain(ctx: AppHandle, args: Domain) {
//   match ctx.state::<Forwarder>().get_domain(&args.domain).await {
//     Ok(_) => {},
//     Err(_) => {}
//   }

//   todo!()
// }

#[tauri::command]
pub async fn verify_domain(ctx: AppHandle, args: Domain) -> R<VerifyDomain> {
  match ctx.state::<Forwarder>().verify_domain(&args.domain).await {
    Ok(res) => R::ok_data(res),
    Err(_) => R::fail_none(None)
  }
}

#[tauri::command]
pub async fn delete_domain(ctx: AppHandle, args: Domain) -> R<()> {
  match ctx.state::<Forwarder>().delete_domain(&args.domain).await {
    Ok(_) => R::ok_none(),
    Err(_) => R::fail_none(None)
  }
}

#[tauri::command]
pub async fn add_domain(ctx: AppHandle, args: Domain) -> R<CreateDomain> {
  match ctx.state::<Forwarder>().add_domain(&args.domain).await {
    Ok(res) => R::ok_data(res),
    Err(_) => R::fail_none(None)
  }
}