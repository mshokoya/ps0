use serde::Deserialize;
use serde_json::{from_value, Value};
use surrealdb::sql::Id;
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R, 
    libs::db::{
        accounts::types::Account, 
        domain::types::Domain,
        index::DB
    }
};



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
        Err(_) => {
            return R::fail_none(Some("Failed to register account"))
        }
    }
}

#[derive(Debug, Deserialize)]
struct NewDomainArg {
    domain: String,
}

#[tauri::command]
pub async fn add_domain(ctx: AppHandle, args: Value) -> R<Domain> {
    let args: NewDomainArg = match from_value(args) {
        Ok(acc) => acc,
        Err(_) => {return R::fail_none(Some("failed to parse arg"))}
    };

    let _id = Id::rand().to_string();

    match ctx.state::<DB>().insert_one::<Domain>(
        "domain", 
        &_id, 
        Domain {
            _id: _id.clone(),
            verified: false,
            mx_records: false,
            txt_records: false,
            domain: args.domain,
            message: None
        }
    ).await {
        Ok(domain) => R::ok_data(domain.unwrap()),
        Err(_) => {
            return R::fail_none(Some("Failed to register domain"))
        }
    }
}

#[tauri::command]
pub async fn dummy_fn1(ctx: AppHandle, args: Value) {
    // ctx.state::<DB>().delete("record");
}
#[tauri::command]
pub async fn dummy_fn2(ctx: AppHandle, args: Value) {

}
#[tauri::command]
pub async fn dummy_fn3(ctx: AppHandle, args: Value) {

}