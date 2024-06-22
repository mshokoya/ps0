// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod actions;
mod libs;

use actions::apollo::{
    confirm::index::confirm_task,
    update::index::{update_account, update_metadata, update_record},
    delete::index::{delete_account, delete_domain, delete_metadatas, delete_records},
    get::index::{get_accounts, get_domains, get_metadatas, get_records},
    domain::index::{register_domain, verify_domain, remove_domain},
    check::index::check_task,
    demine::index::demine_task,
    login::index::login_task,
    create::index::create_task,
    scrape::index::scrape_task,
    cache::accounts_in_use,
    add::index::{add_account, add_domain, dummy_fn1, dummy_fn2, dummy_fn3}
};
use libs::{cache::ApolloCache, forwarder::index::Forwarder, taskqueue::index::TaskQueue};
use libs::{db::index::DB, scraper::Scraper};
use libs::imap::IMAP;
use once_cell::sync::Lazy;
use std::env;
use tauri::Manager;

static mut SCRAPER: Lazy<Scraper> = Lazy::new(|| Scraper::new());

#[async_std::main]
async fn main() {

    tauri::Builder::default()
    .setup(|app| {
        // cache
        app.manage(ApolloCache::new());
        
        // forwarder
        app.manage(Forwarder("".to_string()));

        // db
        let db = DB::new()?;

        app.manage(db);

        // imap
        app.manage(IMAP::new());
        
        let app_handle = app.app_handle().clone();
        std::panic::set_hook(Box::new(move |info| {
            println!("{}", info);
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
            delete_account,
            delete_domain,
            delete_metadatas,
            delete_records,
            update_account,
            update_metadata,
            update_record,
            demine_task,
            login_task,
            create_task,
            confirm_task,
            scrape_task,
            register_domain, 
            verify_domain, 
            remove_domain,
            accounts_in_use,
            add_account,
            add_domain,
            dummy_fn1,
            dummy_fn2,
            dummy_fn3
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
