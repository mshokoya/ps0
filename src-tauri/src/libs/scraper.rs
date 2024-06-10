use std::{convert::Infallible, net::SocketAddr};
use async_std::task::{block_on, spawn, JoinHandle};
use chromiumoxide::{cdp::browser_protocol::network::{EventRequestWillBeSent, Headers, SetExtraHttpHeadersParams}, error::CdpError, Browser, BrowserConfig, Page};
use futures::StreamExt;
use dotenv_codegen::dotenv;
use serde_json::json;

pub struct Scraper {
    pub browser: Option<Browser>,
}

pub struct Pagee {
    page: Page,
    listener: JoinHandle<()>
}

unsafe impl Send for Scraper {}
unsafe impl Sync for Scraper {}

impl Scraper {
    pub fn new() -> Self {
        Self { browser: None }
    }

    pub async fn init(&mut self) {
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
        ctx.new_page("about:blank").await
    }

    pub async fn incog2(&mut self) -> Result<Pagee, CdpError> {
        // page.disable_log().await?.disable_debugger().await?;
        //     page.enable_stealth_mode().await?;
        let ctx = self
            .browser
            .as_mut()
            .unwrap()
            .start_incognito_context()
            .await?;

        let page = ctx.new_page("about:blank").await?;
        let mut request_will_be_sent = page.event_listener::<EventRequestWillBeSent>().await?.fuse();
        let page_clone = page.clone();
        let listener = async_std::task::spawn(async move {
            while let Some(_) = request_will_be_sent.next().await {
                let _ = page_clone.execute(
                    SetExtraHttpHeadersParams::new(Headers::new(json!({
                        "Proxy-Authorization": dotenv!("FS_B")
                    })))
                    ).await;
            }
        });

        Ok(
            Pagee {
                page,
                listener
            }
        )
    }
}
