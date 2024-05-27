// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod actions;
mod libs;

use actions::apollo::{
    update::index::update_account,
    delete::index::delete_accounts,
    get::index::get_accounts,
    check::index::check_task,
    demine::index::demine_task,
    login::index::login_task,
    create::index::create_task
};
use async_std::task::sleep;
use libs::taskqueue::index::TaskQueue;
use libs::{db::index::DB, scraper::Scraper};
use libs::imap::IMAP;
use once_cell::sync::Lazy;
use polodb_core::bson::oid::ObjectId;
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tauri::Manager;

static mut SCRAPER: Lazy<Scraper> = Lazy::new(|| Scraper::new());

#[async_std::main]
// https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
async fn main() {

    let test = IMAP::new();
    let w = test.watch().await;
    loop {
        let tess = w.recv().await.unwrap();
        let mut output = File::create("file.txt").unwrap();
        let string = format!("{}", tess.body.unwrap());
        output.write(string.to_string().as_bytes());
        // write!(output, "{}", string);
    };

    tauri::Builder::default()
    .setup(|app| {

        std::panic::set_hook(Box::new(move |info| {
            println!("WE IN DA PANIC THINGY !!!!!!!!!");
            println!("{}", info);
            println!("WE IN DA PANIC THINGY !!!!!!!!!");
        }));

        // db
        let db = DB::new();
        db.init();
        app.manage(db);

        // imap
        app.manage(IMAP::new());

        // scraper
        unsafe { SCRAPER.init() };

        // ctx
        let app_handle = app.app_handle().clone();
        app.manage(TaskQueue::new(app_handle));
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![
            check_task, 
            get_accounts, 
            delete_accounts,
            update_account,
            demine_task,
            login_task,
            create_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
