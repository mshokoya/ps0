use std::borrow::{Borrow, BorrowMut};
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_std::task::sleep;
use polodb_core::bson::{doc, to_bson, Uuid};
use serde_json::{ from_value, to_value, Value};
use tauri::{AppHandle, Manager};

use crate::actions::controllers::TaskType;
use crate::libs::imap::IMAP;
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::{
        db::{
            entity::Entity,
            index::DB,
        },
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

// use super::types::ApolloCreateArgs;

#[tauri::command]
pub fn create_task(ctx: AppHandle, args: Value) -> R {
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new(),
        task_type: TaskType::ApolloLogin,
        task_group: TaskGroup::Apollo,
        message: "Demine account popups",
        metadata,
        timeout: None,
        args: Some(args),
    });

    R::ok_none()
}

pub async fn apollo_create(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    // ctx.page = Some(unsafe { SCRAPER.incog().await? });

    let mai = ctx.handle.state::<IMAP>();
    mai.watch().await;

    Ok(None)
}