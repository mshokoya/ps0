use tauri::{AppHandle, Manager};

use crate::{actions::controllers::Response2, libs::cache::ApolloCache};


#[tauri::command]
pub async fn accounts_in_use(ctx: AppHandle) -> Response2<Vec<String>> {
  Response2 {
    ok: true,
    message: None,
    data: ctx.state::<ApolloCache>().get_all_account_ids().await
  }
}