use std::time::Duration;

use anyhow::{anyhow, Result};
use async_std::task::sleep;
use polodb_core::bson::{doc, to_bson, Uuid};
use serde_json::{ from_value, to_value, Value};
use tauri::{AppHandle, Manager};

use crate::actions::apollo::lib::index::apollo_login_credits_info;
use crate::actions::controllers::TaskType;
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

use super::types::ApolloLoginArgs;

#[tauri::command]
pub fn login_task(ctx: AppHandle, args: Value) -> R {
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

pub async fn apollo_login(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    ctx.page = Some(unsafe { SCRAPER.incog().await? });

    ctx.page.as_ref().unwrap().goto("https://app.apollo.io/#/login").await?;

    loop {
      sleep(Duration::from_secs(3)).await;

      let url = ctx.page.as_ref().unwrap().url().await;

      if url.is_err() {
        return Err(anyhow!("page closed early"));
      }

      if url?.unwrap().contains("#/settings/credits/current") {
        let credits = apollo_login_credits_info(&ctx).await?;
        let args: ApolloLoginArgs = from_value(args.unwrap())?;
        let _ = ctx.handle.state::<DB>().update_one(
          Entity::Account, 
          doc! {"_id": args.account_id }, 
          doc! { "$set" : to_bson(&credits).unwrap() }
        );
        return Ok(Some(to_value(credits)?))
      }
    }
}