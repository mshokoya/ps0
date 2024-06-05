use serde::Deserialize;
use serde_json::{from_value, Value};
use surrealdb::sql::Id;
use tauri::{AppHandle, Manager};

use crate::{actions::controllers::Response as R, libs::db::{accounts::types::Account, index::DB}};

#[derive(Debug, Deserialize)]
struct NewAccArg {
    email: String,
    password: String
}

#[tauri::command]
pub async fn add_account(ctx: AppHandle, args: Value) -> R<Account> {

    let args: NewAccArg = match from_value(args) {
        Ok(acc) => acc,
        Err(_) => { return R::fail_none(None)}
    };

    let domain = args.email.split("@").collect::<Vec<&str>>()[1].to_string();
    let login_type = if domain.contains("gmail") || domain.contains("outlook") || domain.contains("hotmail") {
        domain.split(".").collect::<Vec<&str>>()[0].to_string()
    } else {
        "default".to_string()
    };

    let id = Id::rand();



    let db = ctx.state::<DB>();
    let db_guard = db.0.lock().await;

    
    println!("WE ADDING ACCOUNT");
    println!("{:#?}", db);
    println!("{:#?}", db_guard);


        match db_guard.create::<Option<Account>>(("account", id.clone()))
        .content(Account {
            id: id.to_string(), 
            email: args.email,
            password: args.password,
            domain,
            trial_time: None,
            login_type,
            suspended: false,
            verified: "no".to_string(),
            credits_used: None,
            credit_limit: None,
            renewal_date: None,
            renewal_start_date: None,
            renewal_end_date: None,
            last_used: None,
            cookies: None,
            history: vec![],
            total_scraped_recently: 0,
        }).await {
            Ok(account) => {
                println!("WE ADDING ACCOUNT");
                println!("{:#?}", account);
                account
            },
            Err(_) => return R::fail_none(Some("Failed to register account"))
        };

    R::fail_none(None)
    
    // match db {
    //     Some(account) => {
    //         println!("WE ADDING ACCOUNT");
    //         println!("{:#?}", account);
    //         R::ok_data(account)
    //     },
    //     None => R::fail_none(None)
    // }
}

// pub _id: String,
// pub domain: String, // enum Domain
// pub trial_time: Option<u64>,
// pub suspended: bool,
// pub login_type: String, // enum
// pub verified: String, // yes, no, confirm
// pub email: String,
// pub password: String,
// pub proxy: Option<String>,
// pub credits_used: Option<u16>,
// pub credit_limit: Option<u16>,
// pub renewal_date: Option<String>,
// pub renewal_start_date: Option<String>,
// pub renewal_end_date: Option<String>,
// pub last_used: Option<u64>,
// pub cookies: Option<Cookies>,
// pub history: Vec<History>,
// pub total_scraped_recently: u16