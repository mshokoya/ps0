use std::{convert::Infallible, net::SocketAddr};
use headers::{authorization::Basic, Authorization};
use async_std::task::{block_on, spawn, JoinHandle};
use chromiumoxide::{error::CdpError, Browser, BrowserConfig, Page};
use futures::StreamExt;
use hyper::{client::HttpConnector, service::{make_service_fn, service_fn}, Body, Client, Request, Response, Server, StatusCode};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use dotenv_codegen::dotenv;

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
}
