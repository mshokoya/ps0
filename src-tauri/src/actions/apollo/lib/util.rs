use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use async_std::{prelude::FutureExt, task};
use chromiumoxide::{Element, Page};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use url_build_parse::{build_url, parse_url};
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
    pub credit_limit: u16,
    pub renewal_date: String,
    pub renewal_start_date: String,
    pub renewal_end_date: String,
    pub trial_days_left: Option<String>,
}

// ===========================

pub async fn inject_cookies(page: &Page, cookies: &Option<Cookies>) -> Result<()> {
    if cookies.is_some() {
        page.goto("http://www.google.com").await?;
        page.set_cookies(cookies.clone().unwrap().into()).await?;
    }
    Ok(())
}

pub async fn get_browser_cookies(page: &Page) -> Result<String> {
    to_string(&page.get_cookies().await?).context("failed to serialize cookies")
}

pub async fn goto_wait_for_selector(page: &Page, url: &str, selector: &str, interval: u8, delay_secs: u64) -> Result<Element> {
    let _ = page.goto(url).timeout(Duration::from_secs(5)).await;
    wait_for_selector(page, selector, interval, delay_secs).await
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

pub fn set_range_in_url(url: &str, chunk: [u32; 2]) -> String {
    let mut p_url = parse_url(url);
    let p2_url = p_url.as_mut().unwrap();
    p2_url.query.as_mut().unwrap().insert("organizationNumEmployeesRanges[]".to_string(), format!("{}%2C{}", chunk[0], chunk[1]));
    build_url(p_url.unwrap()).unwrap().to_string()
}

pub fn set_page_in_url(url: &str, page: u8) -> String {
    let mut p_url = parse_url(url);
    let p2_url = p_url.as_mut().unwrap();
    p2_url.query.as_mut().unwrap().insert("page".to_string(), page.to_string());
    build_url(p_url.unwrap()).unwrap().to_string()
}

pub fn get_page_in_url(url: &str) -> Option<u8> {
    match parse_url(url).unwrap().query.unwrap().get("page") {
        Some(page) => Some(page.parse::<u8>().unwrap()),
        None => None
    }
}