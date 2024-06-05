use std::time::Duration;

use anyhow::Result;
use async_std::task::sleep;
use polodb_core::bson::{doc, to_document, Uuid};
use serde_json::{to_value, Value};
use tauri::{AppHandle, Manager};
use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo, log_into_apollo_then_visit};
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::{Account};
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::{Response as R},
    libs::{
        db::{
            entity::Entity,
            index::DB,
        },
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

use super::types::ApolloDemineArgs;

#[tauri::command]
pub fn demine_task(ctx: AppHandle, args: Value) -> R {
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new(),
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

    let account = db
        .find_one::<Account>(Entity::Account, Some(doc! {"_id": &args.account_id}))
        .unwrap();

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
            doc! {"taskID": ctx.task_id, "message": format!("successfully obtained {} credits info", account.email)},
        )
        .unwrap();

    Ok(None)
}