use async_std::task::{block_on, spawn};
use chromiumoxide::{error::CdpError, Browser, BrowserConfig, Page};
use futures::StreamExt;

pub struct Scraper {
    pub browser: Option<Browser>,
    // handler: Option<Handler>,
}

unsafe impl Send for Scraper {}
unsafe impl Sync for Scraper {}

impl Scraper {
    pub fn new() -> Self {
        Self { browser: None }
    }

    pub fn init(&mut self) {
        let (browser, mut handler) = block_on(async {
            Browser::launch(BrowserConfig::builder().with_head().build().unwrap())
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
        let ctx = self
            .browser
            .as_mut()
            .unwrap()
            .start_incognito_context()
            .await
            .unwrap();
        ctx.new_page("https://www.google.com").await
    }
}

// use std::sync::{Arc, Mutex};

// use chromiumoxide::{error::CdpError, Browser, Page};
// // use futures::FutureExt;
// // use futures::StreamExt;

// pub struct Scraper {
//     pub browser: Browser,
// }

// unsafe impl Send for Scraper {}
// unsafe impl Sync for Scraper {}

// impl Scraper {
//     pub fn new(browser: Browser) -> Self {
//         Self { browser }
//     }

//     pub async fn incog(&mut self) -> Result<Page, CdpError> {
//         let ctx = self.browser.start_incognito_context().await.unwrap();
//         ctx.new_page("https://www.google.com").await
//     }
// }

// =====================
// ===================
// ====================

// use std::sync::{Arc, Mutex};

// use anyhow::Error;
// use chromiumoxide::{error::CdpError, Browser, Page};

// pub struct Scraper {
//     pub browser: Arc<Mutex<Browser>>,
// }

// impl Scraper {
//     pub fn new(browser: Browser) -> Self {
//         Self {
//             browser: Arc::new(Mutex::new(browser)),
//         }
//     }

//     pub async fn incog_browser(&mut self) -> Result<Page, CdpError> {
//         self.browser
//             .clone()
//             .lock()
//             .unwrap()
//             .start_incognito_context()
//             .await?
//             .new_page("https://www.google.com")
//             .await
//     }
// }
