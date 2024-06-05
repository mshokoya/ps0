use tauri::{AppHandle, Manager};

use crate::{actions::controllers::Response, libs::cache::ApolloCache};


#[tauri::command]
pub async fn accounts_in_use(ctx: AppHandle) -> Response<Vec<String>> {
  Response::ok_data(ctx.state::<ApolloCache>().get_all_account_ids().await)
}