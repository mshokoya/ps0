use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use async_std::{future::timeout, sync::Mutex, task::{block_on, spawn}};
use chromiumoxide::{error::CdpError, Browser, BrowserConfig, Page};
use futures::StreamExt;
use dotenv_codegen::dotenv;
use once_cell::sync::Lazy;
use tauri::api::process::{Command, CommandEvent};
use regex::Regex;

pub struct Scraper {
    pub browser: Option<Browser>,
    pub wait_lock: Arc<Mutex<()>>
    // handler: Option<Handler>,
}

unsafe impl Send for Scraper {}
unsafe impl Sync for Scraper {}

static WAIT: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

impl Scraper {
    pub fn new() -> Self {
        Self { browser: None, wait_lock: Arc::new(Mutex::new(())) }
    }

    pub async fn init(&mut self) {
        let (mut rx, mut child) = Command::new_sidecar("node")
        .expect("failed to create `my-sidecar` binary command")
        .args(vec!["resources/proxy-server.js", dotenv!("FS_PXY_HTTP")])
        .spawn()
        .expect("Failed to spawn sidecar");

        // (FIX impl better error handeling)
        let addr = timeout(Duration::from_secs(20), async move {
            let address_regex = Regex::new(r"(?<address>http:\/\/127\.0\.0\.1:[0-9]*)").unwrap();
            
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("[server] {:?}", line);
                        match address_regex.captures(&line) {
                            Some(cpt) => {
                                return cpt["address"].to_string()
                            },
                            None => {}
                        };
                    }
                    CommandEvent::Stderr(line) => {
                        println!("[server] {:?}", line);
                    }
                    _ => {}
                }
            }
            panic!("failed to find proxy server addr")
        }).await.unwrap();

        
        let (browser, mut handler) = block_on(async {
            Browser::launch(BrowserConfig::builder()
                .with_head()
                // .chrome_executable("resources/Thorium.app/Contents/MacOS/Thorium")
                .args(vec![
                    format!("--proxy-server={}", addr).as_str(),
                    // "--no-sandbox",
                    // "--disable-setuid-sandbox",
                    // "--disable-dev-shm-usage",
                    // "--disable-accelerated-2d-canvas",
                    // "--no-first-run",
                    // "--no-zygote",
                    // // "--single-process", // not working in head browser... try headless
                    // "--disable-gpu",
                    // "--deterministic-fetch",
                    // "--disable-features=IsolateOrigins",
                    // "--disable-site-isolation-trials",
                    // "--disable-features=site-per-process"
                ])
                // .disable_cache()
                .viewport(None)
                .disable_request_intercept()
                // .incognito()
                .build()
                .unwrap())
                .await
                .unwrap()
        });

        spawn(async move {
            loop {
                let _event = handler.next().await.unwrap();
            }
        });

        self.browser = Some(browser);
        // self.handler = Some(handler);
    }

    pub async fn incog(&mut self) -> Result<Page, CdpError> {
        // page.disable_log().await?.disable_debugger().await?;
        //     page.enable_stealth_mode().await?;
        let wait =  WAIT.lock().await;
        if self.browser.is_none() {
            self.init().await;
        }
        drop(wait); 

        let ctx = self
            .browser
            .as_mut()
            .unwrap()
            .start_incognito_context()
            .await
            .unwrap();

        let page = ctx.new_page("https://www.google.com").await?;
        page.enable_stealth_mode().await?;
        page
            .disable_css().await?
            .disable_debugger().await?
            .disable_log().await?;
            // .disable_runtime().await?;
        
        Ok(page)
    }
}
