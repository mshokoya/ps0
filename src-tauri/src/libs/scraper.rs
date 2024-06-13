use std::{convert::Infallible, net::SocketAddr};
use headers::{authorization::Basic, Authorization};
use async_std::task::{block_on, spawn, JoinHandle};
use chromiumoxide::{error::CdpError, Browser, BrowserConfig, Page};
use futures::StreamExt;
use hyper::{client::HttpConnector, service::{make_service_fn, service_fn}, Body, Client, Request, Response, StatusCode};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use dotenv_codegen::dotenv;

use crate::PS_ADDR;

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
        let ps_addr =  unsafe { PS_ADDR.get().unwrap() };
        
        let (browser, mut handler) = block_on(async {
            Browser::launch(BrowserConfig::builder()
                .with_head()
                .args(vec![
                    format!("
                    --proxy-server={}", ps_addr).as_str(),
                    "--no-sandbox",
                    "--disable-setuid-sandbox",
                    "--disable-dev-shm-usage",
                    "--disable-accelerated-2d-canvas",
                    "--no-first-run",
                    "--no-zygote",
                    // "--single-process", // not working in head browser... try headless
                    "--disable-gpu",
                    "--deterministic-fetch",
                    "--disable-features=IsolateOrigins",
                    "--disable-site-isolation-trials",
                    "--disable-features=site-per-process"
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


// struct ProxyServer(pub JoinHandle<()>);

// impl ProxyServer {

//     pub async fn new() -> Self {
//         ProxyServer(
//             spawn(async {
//                 let addr = SocketAddr::from(([127, 0, 0, 1], 8100));

//                 let client = Client::builder()
//                     .http1_title_case_headers(true)
//                     .http1_preserve_header_case(true)
//                     .build_http();

//                 let make_service = make_service_fn(move |_| {
//                     let client = client.clone();
//                     async move { Ok::<_, Infallible>(service_fn(move |req| Self::proxy(client.clone(), req))) }
//                 });

//                 let server = Server::bind(&addr)
//                     .http1_preserve_header_case(true)
//                     .http1_title_case_headers(true)
//                     .serve(make_service);

//                 println!("Listening on http://{}", addr);

//                 if let Err(e) = server.await {
//                     eprintln!("server error: {}", e);
//                 };
//             })
//         )
//     }

//     async fn proxy_client() {
//         let mut proxy = Proxy::new(Intercept::All, dotenv!("FS_PXY").parse().unwrap());
//         proxy.set_authorization(Authorization::basic(dotenv!("FS_UN"), dotenv!("FS_PW")));
//         let connector = HttpConnector::new();
//         let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
        
//         Client::builder()
//             .build(proxy);
//     }

//     // async fn proxy(_client: Client<HttpConnector>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     //     let headers = req.headers().clone();
//     //     println!("headers: {:?}", headers);
//     //     let path = req.uri().path().to_string();
//     //     let resp = Self::get_response(_client, req).await?;
//     //     Ok(resp)
//     // }

//     async fn get_response(client: Client<HttpConnector>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//         let headers = req.headers().clone();
//         let mut request_builder = Request::builder()
//             .method(req.method())
//             .uri(req.uri())
//             .body(req.into_body())
//             .unwrap();
    
//         *request_builder.headers_mut() = headers;
//         let response = client.request(request_builder).await?;
//         let body = hyper::body::to_bytes(response.into_body()).await?;
//         let body = String::from_utf8(body.to_vec()).unwrap();
    
//         let mut resp = Response::new(Body::from(body));
//         *resp.status_mut() = StatusCode::OK;
//         Ok(resp)
//     }
// }