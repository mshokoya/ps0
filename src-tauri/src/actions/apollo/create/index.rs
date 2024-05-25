use std::time::Duration;
use async_std::future::{timeout};
use polodb_core::bson::oid::ObjectId;
use simple_password_generator::PasswordGenerator;
use anyhow::{anyhow, Result};
use async_std::task::sleep;
use polodb_core::bson::{doc, to_bson, to_document, Uuid};
use serde_json::{from_value, Value};
use tauri::{AppHandle, Manager};
use faker::Faker;
use regex::Regex;

use crate::actions::apollo::lib::util::wait_for_selector;
use crate::actions::controllers::TaskType;
use crate::libs::db::accounts::types::Account;
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

use super::types::ApolloCreateArgs;

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
    ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    let args: ApolloCreateArgs = from_value(args.unwrap())?;

    let faker = Faker::new("us");
    let email = format!("{}@{}", faker.lorem.word(), args.domain); // (FIX) faker.lorem.word() nor sure of length
    let password = PasswordGenerator::new().length(20).generate();

    let imap = ctx.handle.state::<IMAP>();
    let receiver = imap.watch().await;

    let page = unsafe { SCRAPER.incog().await? };

    page.goto("https://www.apollo.io/sign-up").await?;

    let input = wait_for_selector(&page, r#"input[class="MuiInputBase-input MuiOutlinedInput-input mui-style-1x5jdmq"]"#, 10, 3).await?;
    input.click().await?.type_str(&email).await?;

    let checkbox =  page.find_element(r#"input[class="PrivateSwitchBase-input mui-style-1m9pwf3"]"#).await?;
    checkbox.click().await?.type_str(&password).await?;

    let signup = page.find_element(r#"[class="MuiBox-root mui-style-1tu59u4"]"#).await?;
    signup.click().await?.type_str(&password).await?;

    for _ in 0..10 {
        sleep(Duration::from_secs(2)).await;

        let error = page.find_element(r#"p[class="MuiTypography-root MuiTypography-bodySmall mui-style-1ccelp7"]"#).await;
        if error.is_ok() {
            return Err(anyhow!("Failed to signup, error in email"));
        } 
        
        if page.url().await?.unwrap().contains("sign-up/success") {
            break 
        }
    }

    ctx.handle.state::<DB>().insert_one(
        Entity::Account, 
        to_document(&Account {
            _id: ObjectId::new().to_hex(),
            domain: args.domain,
            trial_time: None,
            suspended: false,
            login_type: "default".to_string(), // (FIX) make it dynamic
            verified: "confirm".to_string(),
            email: email.clone(),
            password: password.clone(),
            proxy: None,
            credits_used: None,
            credit_limit: None,
            renewal_date: None,
            renewal_start_date: None,
            renewal_end_date: None,
            last_used: None,
            cookies: None,
            history: vec![]
        })?
    )?;

    let confirmation_link = timeout(
        Duration::from_secs(60), 
        async move {
            let re1 = Regex::new(r"/(?<=Activate Your Account \( )[\S|\n]+/g").unwrap();
            let re2 = Regex::new(r"/(?<=Or paste this link into your browser: )[\S|\n]+(?= \()/g").unwrap();
            loop {
                let envelope = receiver.recv().await.unwrap();
                if envelope.from.as_ref().unwrap()[0].name.as_ref().unwrap().contains("apollo") {
                    let body = envelope.body.unwrap();
                    if body.contains("Activate Your Account") {
                        let link1 = re1.find(&body).unwrap();
                        let _link2 = re2.find(&body).unwrap();
        
                        return link1.as_str().to_string();
                    }
                }
            }
        }
    ).await?;

    page.goto(confirmation_link).await?;
    // wait_for_selector(&page, r#"input[class="MuiInputBase-input MuiOutlinedInput-input mui-style-1x5jdmq"]"#, 10, 3).await?;
    let _name =  wait_for_selector(&page, r#"input[class="zp_bWS5y zp_J0MYa"][name="name"]"#, 10, 2).await?.click().await?.type_str(faker.name.full_name()).await?;
    let _password = page.find_element(r#"input[class="zp_bWS5y zp_J0MYa"][name="password"]"#).await?.click().await?.type_str(&password).await?;
    let _confirm_password = page.find_element(r#"input[class="zp_bWS5y zp_J0MYa"][name="confirmPassword"]"#).await?.click().await?.type_str(&password).await?;
    let _submit = page.find_element(r#"button[class="zp-button zp_zUY3r zp_aVzf8"]"#).await?.click().await?.type_str(&password).await?;

    let mut counter = 0;
    while counter <= 5 {
        let onboarding = page.find_element(r#"[class="zp-button zp_zUY3r zp_OztAP zp_lshSd"]"#).await.ok();
        let skip = page.find_element(r#"[class="zp-button zp_zUY3r zp_MCSwB"]"#).await.ok();
        let new_team = page.find_element(r#"button[class="zp-button zp_zUY3r zp_MCSwB zp_OztAP zp_LUHm0"][type="button"]"#).await.ok();
        let close = page.find_element(r#"[class="zp-icon mdi mdi-close zp_dZ0gM zp_foWXB zp_j49HX zp_rzbAy"]"#).await.ok();
        let url = page.url().await?.unwrap();

        if new_team.is_some() {
            new_team.unwrap().click().await?;
            counter = 0;
        } else if onboarding.is_some() {
            onboarding.unwrap().click().await?;
            counter = 0;
        } else if skip.is_some() {
            skip.unwrap().click().await?;
            counter = 0;
        } else if close.is_some() {
            close.unwrap().click().await?;
            counter = 0;
        } else if url.contains("signup-success") {
            page.goto("https://app.apollo.io/").await?;
            counter = 0;
        } else if 
            url.contains("app.apollo.io/#/onboarding-hub/queue") ||
            url.contains("app.apollo.io/#/control-center") ||
            url.contains("app.apollo.io/#/sequences") ||
            url.contains("app.apollo.io/#/conversations") ||
            url.contains("app.apollo.io/#/opportunities") ||
            url.contains("app.apollo.io/#/enrichment-status") ||
            url.contains("app.apollo.io/#/settings")
        {
            break
        }
    }
    Ok(None)
}