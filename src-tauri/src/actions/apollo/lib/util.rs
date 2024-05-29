use std::time::Duration;

use anyhow::{anyhow, Result};
use async_std::task;
use chromiumoxide::{Element, Page};
use serde::{Deserialize, Serialize};

use crate::libs::db::accounts::types::Cookies;

// ===== Apollo Error ======
// #[derive(Debug)]
// pub struct ApolloError {
//     task_id: Uuid,
//     message: &'static str,
// }
// impl ApolloError {
//     pub fn new(task_id: Uuid, message: &str) -> Self {
//         ApolloError {
//             task_id,
//             message,
//         }
//     }
// }

// impl std::error::Error for ApolloError {}

// impl std::fmt::Display for ApolloError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[ApolloError]: {}", self.message)
//     }
// }
// ==========================
//
// =======================
// ======= creditsInfo ========

#[derive(Debug, Deserialize)]
pub struct CreditsEval {
    pub email_credits_info: String,
    pub renewal_date: String,
    pub renewal_start_end: String,
    pub trial_days_left: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditsInfo {
    pub credits_used: u16,
    pub credits_limit: u16,
    pub renewal_date: String,
    pub renewal_start_date: String,
    pub renewal_end_date: String,
    pub trial_days_left: Option<String>,
}

// ===========================

pub async fn inject_cookies(page: &Page, cookies: &Option<Cookies>) -> Result<()> {
    if cookies.is_some() {
        page.goto("http://www.google.com").await?;
        page.wait_for_navigation_response().await?;
        page.set_cookies(cookies.clone().unwrap().into()).await?;
    }
    Ok(())
}

pub async fn wait_for_selectors(
    page: &Page,
    selector: &str,
    mut interval: u8,
    delay_secs: u8,
) -> Result<Vec<Element>> {
    let selector: String = selector.into();
    while interval > 0 {
        let el: Option<Vec<Element>> = page.find_elements(&selector).await.ok();
        if el.is_some() {
            return Ok(el.unwrap());
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs.into())).await;
    }

    Err(anyhow!(
        "[Error - wait_for_selectors]: Failed to find selectors",
    ))
}

pub async fn wait_for_selector(
    page: &Page,
    selector: &str,
    mut interval: u8,
    delay_secs: u64,
) -> Result<Element> {
    let selector: String = selector.into();
    while interval > 0 {
        let el: Option<Element> = page.find_element(&selector).await.ok();
        
        if el.is_some() {
            return Ok(el.unwrap());
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs)).await;
    }

    Err(anyhow!(
        "[Error - wait_for_selector]: Failed to find selector"
    ))
}

pub fn time_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}