use std::time::Duration;
use async_std::future::timeout;
use fake::faker::internet::en::Username;
use fake::faker::name::en::{FirstName, LastName};
use simple_password_generator::PasswordGenerator;
use anyhow::{anyhow, Result};
use async_std::task::sleep;
use serde_json::{from_value, json as to_serde_json, Value};
use surrealdb::sql::{to_value, Id, json};
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

use super::types::ApolloCreateArgs;

#[tauri::command]
pub fn create_task(ctx: AppHandle, args: Value) -> R<()> {
    let metadata = match args.get("domain") {
        Some(val) => Some(to_serde_json!({"domain": val.clone()})),
        None => None,
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Id::uuid().to_string(),
        task_type: TaskType::ApolloCreate,
        task_group: TaskGroup::Apollo,
        message: "Creating account",
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

    let args: ApolloCreateArgs = from_value(args.unwrap())?;
    let email = format!("{}@{}", Username().fake::<String>(), args.domain); // (FIX) faker.lorem.word() nor sure of length
    let password = PasswordGenerator::new().length(25).generate();
    let imap = ctx.handle.state::<IMAP>();
    ctx.page =  Some(unsafe { SCRAPER.incog().await? });
    let page = ctx.page.as_ref().unwrap();

    page.goto("https://www.apollo.io/sign-up").await?;
    
    match wait_for_selector(&page, r#"[class="CybotCookiebotDialogBodyButton"]"#, 10, 3).await {
        Ok(el) => {
            el.focus().await?.click().await?;
        }
        Err(_) => {}
    }

    let _input = wait_for_selector(&page, r#"input[class="MuiInputBase-input MuiOutlinedInput-input mui-style-1x5jdmq"]"#, 5, 3).await?.focus().await?.type_str(&email).await?;
    let _checkbox =  wait_for_selector(&page, r#"input[class="PrivateSwitchBase-input mui-style-1m9pwf3"]"#, 5, 3).await?.focus().await?.click().await?;
    let _signup = wait_for_selector(&page, r#"[class="MuiTypography-root MuiTypography-body1 mui-style-3kpuwu"]"#, 5, 3).await?.focus().await?.click().await?;

    for _ in 0..10 {
        sleep(Duration::from_secs(2)).await;

        let error = page.find_element(r#"[class="MuiTypography-root MuiTypography-bodySmall mui-style-1ccelp7"]"#).await;
        if error.is_ok() {
            return Err(anyhow!("Failed to signup, error in email"));
        } 
        
        if page.url().await?.unwrap().contains("sign-up/success") {
            break 
        }
    }

    let account_id = Id::rand().to_string();
    let db = ctx.handle.state::<DB>();

    let _ = db.insert_one::<Account>(
        "account", 
        &account_id,
            to_value(
                Account {
                _id: account_id.clone(),
                domain: args.domain,
                trial_time: None,
                suspended: false,
                login_type: "default".to_string(), // (FIX) make it dynamic
                verified: "confirm".to_string(),
                email: email.clone(),
                password: password.clone(),
                credits_used: None,
                credit_limit: None,
                renewal_date: None,
                renewal_start_date: None,
                renewal_end_date: None,
                last_used: None,
                cookies: None,
                history: vec![],
                total_scraped_recently: 0
            })?
    );

    sleep(Duration::from_secs(15)).await;
    let receiver = imap.watch().await;
    let confirmation_link = match timeout(
        Duration::from_secs(60), 
        async move {
            loop {
                let envelope = receiver.recv().await.unwrap();

                if envelope.to.unwrap().contains(&email) {
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
            let _ = ctx.handle.state::<DB>()
            .update_one::<Account>(
                "account", 
                &account_id, 
                json(r#"{"verified": "yes"}"#)?
                // to_value(json!({"verified": "yes"}))?
            ).await?;
            return Ok(None)
        }
        sleep(Duration::from_secs(3)).await;
        counter += 1;
    }
    Err(anyhow!("Failed to signup: please try again"))
}

// more teams drop down
// button[class="zp-button zp_zUY3r zp_B5hnZ zp_LZAms"]

// start new team button
// button[class="zp-button zp_zUY3r zp_MCSwB zp_LUHm0"]
// ================

//  do not click join team button
// zp-button zp_zUY3r zp_OztAP zp_wlMPY