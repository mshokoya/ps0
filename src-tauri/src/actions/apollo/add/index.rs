use serde::{Deserialize, Serialize};
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
        Err(_) => {return R::fail_none(None)}
    };

    let domain = args.email.split("@").collect::<Vec<&str>>()[1].to_string();
    let login_type = if domain.contains("gmail") || domain.contains("outlook") || domain.contains("hotmail") {
        domain.split(".").collect::<Vec<&str>>()[0].to_string()
    } else {
        "default".to_string()
    };

    let _id = Id::rand().to_string();

    match ctx.state::<DB>().insert_one::<Account>(
        "account", 
        &_id, 
        Account {
            _id: _id.clone(),
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
        }
    ).await {
        Ok(account) => R::ok_data(account.unwrap()),
        Err(e) => {
            return R::fail_none(Some("Failed to register account"))
        }
    }
}




// ============= update one

// #[tauri::command]
// pub async fn add_account(ctx: AppHandle, args: Value) -> R<Account> {
//     let args: NewAccArg = match from_value(args) {
//         Ok(acc) => acc,
//         Err(_) => {return R::fail_none(None)}
//     };

//     match ctx.state::<DB>().update_one::<Account>("account", "19imojfg9ywmo9bgxcdd", args).await {
//         Ok(docs) => {
//             println!("UPDATE CHECK");
//             println!("{docs:?}");
//             if docs.is_none() {
//             return R::fail_none(Some("Failed to find account"))
//             }
//             R::ok_data(docs.unwrap())
//         },
//         Err(e) =>  {
//             println!("UPDATE err");
//             println!("{e:?}");
//             R::ok_none()
//         }
//     }
// }















// ========================================

// #[derive(Debug, Deserialize)]
// struct NewAccArg {
//     email: String,
//     password: String
// }

// #[tauri::command]
// pub async fn add_account(ctx: AppHandle, args: Value) -> R<Account> {
//     let args: NewAccArg = match from_value(args) {
//         Ok(acc) => acc,
//         Err(_) => {return R::fail_none(None)}
//     };

//     let domain = args.email.split("@").collect::<Vec<&str>>()[1].to_string();
//     let login_type = if domain.contains("gmail") || domain.contains("outlook") || domain.contains("hotmail") {
//         domain.split(".").collect::<Vec<&str>>()[0].to_string()
//     } else {
//         "default".to_string()
//     };

//     let _id = Id::rand().to_string();

//     let db: Option<Account> = match ctx.state::<DB>().0.lock().await
//         .create(("account", _id.clone()))
//         .content(Account {
//             _id,
//             email: args.email,
//             password: args.password,
//             domain,
//             trial_time: None,
//             login_type,
//             suspended: false,
//             verified: "no".to_string(),
//             credits_used: None,
//             credit_limit: None,
//             renewal_date: None,
//             renewal_start_date: None,
//             renewal_end_date: None,
//             last_used: None,
//             cookies: None,
//             history: vec![],
//             total_scraped_recently: 0,
//         }).await {
//             Ok(account) => account,
//             Err(e) => {
//                 return R::fail_none(Some("Failed to register account"))
//             }
//         };
    
//     match db {
//         Some(account) => R::ok_data(account),
//         None => R::fail_none(None)
//     }
// }
