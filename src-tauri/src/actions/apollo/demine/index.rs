use std::time::Duration;

use anyhow::{anyhow, Result};
use async_std::task::sleep;
use serde_json::{json, Value};
use surrealdb::sql::Id;
use tauri::{AppHandle, Manager};
use crate::actions::apollo::lib::index::log_into_apollo;
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::Account;
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::{
        db::index::DB,
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

use super::types::ApolloDemineArgs;

#[tauri::command]
pub fn demine_task(ctx: AppHandle, args: Value) -> R<()> {
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Id::uuid().to_string(),
        task_type: TaskType::ApolloDemine,
        task_group: TaskGroup::Apollo,
        message: "Demine account popups",
        metadata,
        timeout: None,
        args: Some(args),
    });

    R::ok_none()
}

pub async fn apollo_demine(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    let args: ApolloDemineArgs = serde_json::from_value(args.unwrap())?;
    let db = ctx.handle.state::<DB>();

    let Some(account) = db
        .select_one::<Account>(
            "account", 
            &args.account_id).await?
        else { 
            return Err(anyhow!("Failed to find registered account"))
        };

    ctx.page = Some(unsafe { SCRAPER.incog().await? });

    log_into_apollo(&ctx, &account).await?;

    loop {
        sleep(Duration::from_secs(5)).await;

        if ctx.page.as_ref().unwrap().url().await.is_err() {
            break
        }
    }

    ctx.handle
        .emit_all(
            TaskGroup::Apollo.into(),
            json!({
                "taskID": ctx.task_id, 
                "message": format!("successfully obtained {} credits info", account.email)
            })
        )
        .unwrap();

    Ok(None)
}