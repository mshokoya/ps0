use std::time::Duration;
use async_std::future::timeout;
use fake::faker::name::en::{FirstName, LastName};
use serde::Deserialize;
use simple_password_generator::PasswordGenerator;
use anyhow::{anyhow, Result};
use async_std::task::sleep;
use serde_json::{from_value, Value};
use surrealdb::sql::{Id, json};
use tauri::{AppHandle, Manager};
use fake::Fake;
use crate::actions::apollo::lib::util::wait_for_selector;
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::Account;
use crate::libs::imap::IMAP;
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::{
        db::index::DB,
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

#[derive(Deserialize)]
struct ApolloConfirmArgs {
    account_id: String
}

#[tauri::command]
pub fn confirm_task(ctx: AppHandle, args: Value) -> R<()> {
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Id::uuid().to_string(),
        task_type: TaskType::ApolloConfirm,
        task_group: TaskGroup::Apollo,
        message: "Confirm account",
        metadata,
        timeout: None,
        args: Some(args),
    });

    R::ok_none()
}

pub async fn apollo_confirm(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {

    let args: ApolloConfirmArgs = from_value(args.unwrap())?;
    let password = PasswordGenerator::new().length(25).generate();
    let imap = ctx.handle.state::<IMAP>();
    ctx.page =  Some(unsafe { SCRAPER.incog().await? });
    let page = ctx.page.as_ref().unwrap();
    let db = ctx.handle.state::<DB>();

    let account = match db.select_one::<Account>(
        "account",
        &args.account_id,
    ).await? {
        Some(account) => account,
        None => {return Err(anyhow!("Failed to find email"))}
    };

    // match wait_for_selector(&page, r#"[class="CybotCookiebotDialogBodyButton"]"#, 10, 3).await {
    //     Ok(el) => {
    //         el.focus().await?.click().await?;
    //     }
    //     Err(_) => {}
    // }

    let receiver = imap.watch().await;
    let confirmation_link = match timeout(
        Duration::from_secs(60), 
        async move {
            loop {
                let envelope = receiver.recv().await.unwrap();

                if envelope.to.unwrap().contains(&account.email) {
                    if let Some(link) = envelope.link_1 {
                        return link
                    }
                    if let Some(link) = envelope.link_2 {
                        return link
                    }
                }
            }
        }
    ).await {
        Ok(val) => {
            imap.unwatch().await;
            val
        },
        Err(e) => {
            imap.unwatch().await;
            return Err(anyhow!("{}", e.to_string()))
        }
    };

    page.goto(confirmation_link).await?;
    // wait_for_selector(&page, r#"input[class="MuiInputBase-input MuiOutlinedInput-input mui-style-1x5jdmq"]"#, 10, 3).await?;
    let name = format!("{} {}", FirstName().fake::<String>(), LastName().fake::<String>());
    let _name =  wait_for_selector(&page, r#"input[class="zp_bWS5y zp_J0MYa"][name="name"]"#, 10, 2).await?.focus().await?.click().await?.type_str(&name).await?;
    let _password = page.find_element(r#"input[class="zp_bWS5y zp_J0MYa"][name="password"]"#).await?.click().await?.focus().await?.type_str(&password).await?;
    let _confirm_password = page.find_element(r#"input[class="zp_bWS5y zp_J0MYa"][name="confirmPassword"]"#).await?.focus().await?.click().await?.type_str(&password).await?;
    let _submit = page.find_element(r#"button[class="zp-button zp_zUY3r zp_aVzf8"]"#).await?.focus().await?.click().await?;

    let mut counter = 0;
    while counter <= 7 {
        let onboarding = page.find_element(r#"[class="zp-button zp_zUY3r zp_OztAP zp_lshSd"]"#).await.ok();
        let skip = page.find_element(r#"[class="zp-button zp_zUY3r zp_MCSwB"]"#).await.ok();
        let new_team_dropdown = page.find_element(r#"button[class="zp-button zp_zUY3r zp_B5hnZ zp_LZAms"]"#).await.ok();
        let new_team = page.find_element(r#"button[class="zp-button zp_zUY3r zp_MCSwB zp_LUHm0"][type="button"]"#).await.ok();
        let close = page.find_element(r#"[class="zp-icon mdi mdi-close zp_dZ0gM zp_foWXB zp_j49HX zp_rzbAy"]"#).await.ok();
        let url = page.url().await?.unwrap();

        if new_team.is_some() {
            new_team.unwrap().focus().await?.click().await?;
            counter = 0;

        } else if new_team_dropdown.is_some() {
            new_team_dropdown.unwrap().focus().await?.click().await?;
            counter = 0;
        } else if onboarding.is_some() {
            onboarding.unwrap().focus().await?.click().await?;
            counter = 0;
        } else if skip.is_some() {
            skip.unwrap().focus().await?.click().await?;
            counter = 0;
        } else if close.is_some() {
            close.unwrap().focus().await?.click().await?;
            counter = 0;
        } else if url.contains("signup-success") {
            page.goto("https://app.apollo.io/").await?;
            counter = 0;
        } else if 
            url.contains("onboarding-hub/queue") ||
            url.contains("control-center") ||
            url.contains("sequences") ||
            url.contains("conversations") ||
            url.contains("opportunities") ||
            url.contains("enrichment-status") ||
            url.contains("settings")
        {
            let _ = db.update_one::<Account>(
                    "account", 
                    &args.account_id, 
                    json(r#"{"verified": "yes"}"#)?
                    // to_value(json!({"verified": "yes"}))?
            ).await?;
            return Ok(None)
        }
        sleep(Duration::from_secs(3)).await;
        counter += 1;
    }
    Err(anyhow!("Failed to signup, please try again"))
}