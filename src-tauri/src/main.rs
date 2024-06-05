// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod actions;
mod libs;

use actions::apollo::{
    confirm::index::confirm_task,
    update::index::{update_account, update_domain, update_metadata, update_record},
    delete::index::{delete_accounts, delete_domains, delete_metadatas, delete_records},
    get::index::{get_accounts, get_domains, get_metadatas, get_records},
    domain::index::{add_domain, verify_domain, delete_domain},
    check::index::check_task,
    demine::index::demine_task,
    login::index::login_task,
    create::index::create_task,
    scrape::index::scrape_task,
    cache::accounts_in_use,
    add::index::add_account
};
use libs::{cache::ApolloCache, forwarder::index::Forwarder, taskqueue::index::TaskQueue};
use libs::{db::index::DB, scraper::Scraper};
use libs::imap::IMAP;
use once_cell::sync::Lazy;
use std::env;
use tauri::Manager;

static mut SCRAPER: Lazy<Scraper> = Lazy::new(|| Scraper::new());

#[async_std::main]
// https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
async fn main() {
    tauri::Builder::default()
    .setup(|app| {
        // cache
        app.manage(ApolloCache::new());
        
        // forwarder
        app.manage(Forwarder("".to_string()));

        // db
        let db = DB::new();
        app.manage(db);

        // imap
        app.manage(IMAP::new());

        // scraper
        unsafe { SCRAPER.init() };

        let app_handle = app.app_handle().clone();
        std::panic::set_hook(Box::new(move |info| {
            println!("{}", info);
            // let state = state;
            async_std::task::block_on(async {
                unsafe { SCRAPER.browser.as_mut().unwrap().close().await.unwrap(); };
                app_handle.state::<IMAP>().logout().await.unwrap();
            })
        }));

        // ctx
        let app_handle = app.app_handle().clone();
        app.manage(TaskQueue::new(app_handle));
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![
            check_task,
            get_accounts,
            get_domains,
            get_metadatas,
            get_records,
            delete_accounts,
            delete_domains,
            delete_metadatas,
            delete_records,
            update_account,
            update_metadata,
            update_domain,
            update_record,
            demine_task,
            login_task,
            create_task,
            confirm_task,
            scrape_task,
            add_domain, 
            verify_domain, 
            delete_domain,
            accounts_in_use,
            add_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
