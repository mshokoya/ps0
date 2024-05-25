use anyhow::Result;
use polodb_core::bson::oid::ObjectId;
use polodb_core::bson::{doc, to_document, Uuid};
use serde_json::{to_value, Value};
use tauri::{AppHandle, Manager};

use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo_then_visit};
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::{Account, AccountArg};
use crate::libs::taskqueue::index::TaskQueue;
use crate::libs::taskqueue::types::Channels;
use crate::{
    actions::controllers::{Response as R},
    libs::{
        db::{
            entity::Entity,
            index::DB,
        },
        taskqueue::types::{TQTimeout, Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

use super::types::ApolloCheckArgs;

#[tauri::command]
pub fn check_task(ctx: AppHandle, args: Value) -> R {
    let to = args.get("timeout").unwrap().to_owned();
    let timeout: Option<TQTimeout> = serde_json::from_value(to).unwrap_or(None);
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.to_owned()),
        None => None,
    };

    let fmt_args = match args.get("account_id") {
        Some(_) => Some(args.to_owned()),
        None => return R::fail_none(),
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new(),
        task_type: TaskType::ApolloCheck,
        task_group: TaskGroup::Apollo,
        message: "Getting credits",
        metadata,
        timeout,
        args: fmt_args,
    });

    R::ok_none()
}

pub async fn apollo_check(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    let args: ApolloCheckArgs = serde_json::from_value(args.unwrap())?;
    let db = ctx.handle.state::<DB>();

    println!("{:?}",args.account_id);

    let account = db
        .find_one::<Account>(Entity::Account, Some(doc! {"_id": args.account_id}))
        .unwrap();

    ctx.page = Some(unsafe { SCRAPER.incog().await? });

    log_into_apollo_then_visit(
        &ctx,
        &account,
        "https://app.apollo.io/#/settings/credits/current",
    )
    .await?;

    ctx.handle
        .emit_all(
            Channels::Apollo.into(),
            doc! {"taskID": ctx.task_id, "message": "navigated to the credits page"},
        )
        .unwrap();

    let update = apollo_login_credits_info(&ctx).await?;

    db.update_one(
        Entity::Account,
        doc! {"_id": ""},
        doc! {"$set": to_document(&update)?},
    )?;

    ctx.handle
        .emit_all(
            Channels::Apollo.into(),
            doc! {"taskID": ctx.task_id, "message": format!("successfully obtained {} credits info", account.email)},
        )
        .unwrap();

    Ok(Some(to_value(update)?))
}