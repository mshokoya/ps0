use anyhow::{anyhow, Context, Result};
use serde_json::{json, to_value as to_serde_value, Value};
use surrealdb::sql::{to_value, Id};
use tauri::{AppHandle, Manager};
use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo_then_visit};
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::Account;
use crate::libs::db::index::DB;
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    SCRAPER,
};

use super::types::ApolloCheckArgs;

#[tauri::command]
pub fn check_task(ctx: AppHandle, mut args: Value) -> R<()> {
    let timeout = match args.get_mut("timeout") {
        Some(val) => serde_json::from_value(val.take()).unwrap_or(None),
        None => None
    };

    let metadata = match args.get("account_id") {
        Some(val) => Some(json!({"account_id": val}) ),
        None => return R::fail_none(Some("account not found")),
    };

    let fmt_args = match args.get("account_id") {
        Some(_) => Some(args.to_owned()),
        None => return R::fail_none(Some("account not found")),
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Id::uuid().to_string(),
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

    let Some(account) = db
    .select_one::<Account>("account", &args.account_id)
    .await
    .context(anyhow!("Failed to check account, could not find registered account"))?
    else {
        return Err(anyhow!("Failed to check account, could not find registered account"))
    };

    ctx.page = Some(unsafe { SCRAPER.incog().await? });

    log_into_apollo_then_visit(
        &ctx,
        &account,
        "https://app.apollo.io/#/settings/credits/current",
    )
    .await?;

    ctx.handle
        .emit_all(
            TaskGroup::Apollo.into(),
            json!({
                "taskID": &ctx.task_id, 
                "message": "navigated to the credits page"
            })
        )?;

    let update = apollo_login_credits_info(&ctx).await?;

    db.update_one::<Account>(
        "account", 
        &args.account_id, 
        to_value(&update)?
    ).await?;

    ctx.handle
        .emit_all(
            TaskGroup::Apollo.into(),
            json!({
                "taskID": ctx.task_id, 
                "message": format!("successfully obtained {} credits info", account.email)
            }),
        )?;

    Ok(Some(to_serde_value(update)?))
}