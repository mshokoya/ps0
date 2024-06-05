use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::{actions::controllers::Response2, libs::forwarder::{index::Forwarder, types::{CreateDomain, VerifyDomain}}};


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
pub async fn verify_domain(ctx: AppHandle, args: Domain) -> Response2<VerifyDomain> {
  match ctx.state::<Forwarder>().verify_domain(&args.domain).await {
    Ok(res) => Response2::ok_data(res),
    Err(_) => Response2 {ok: false, message: None, data: None}
  }
}

#[tauri::command]
pub async fn delete_domain(ctx: AppHandle, args: Domain) -> Response2<()> {
  match ctx.state::<Forwarder>().delete_domain(&args.domain).await {
    Ok(_) => Response2::ok_none(),
    Err(_) => Response2 {ok: false, message: None, data: None}
  }
}

#[tauri::command]
pub async fn add_domain(ctx: AppHandle, args: Domain) -> Response2<CreateDomain> {
  match ctx.state::<Forwarder>().add_domain(&args.domain).await {
    Ok(res) => Response2::ok_data(res),
    Err(_) => Response2 {ok: false, message: None, data: None}
  }
}