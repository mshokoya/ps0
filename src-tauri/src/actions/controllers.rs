use serde::{Deserialize, Serialize};
use serde_json::Value;
// use uuid::Uuid;

use crate::libs::{
    db::accounts::types::AccountArg, taskqueue::types::{TQTimeout, TaskActionCTX}
};

// use super::apollo::{
//     check::index::apollo_check, confirm::index::apollo_confirm, create::index::apollo_create, demine::index::apollo_demine, login::index::apollo_login, scrape::index::apollo_scrape
// };

#[derive(Serialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok_none() -> Self {
        Response::<T> {
            ok: true,
            message: None,
            data: None,
        }
    }

    pub fn ok_data(data: T) -> Self {
        Response {
            ok: true,
            message: None,
            data: Some(data)
        }
    }

    pub fn fail_none(msg: Option<&str>) -> Self {
        Response::<T> {
            ok: false,
            message: msg.and(Some(msg.unwrap().to_string())),
            data: None,
        }
    }
}

// #[derive(Serialize, Debug)]
// pub struct Response {
//     pub ok: bool,
//     pub message: Option<String>,
//     pub data: Option<Value>,
// }

// impl Response {
//     pub fn ok_none() -> Self {
//         Response {
//             ok: true,
//             message: None,
//             data: None,
//         }
//     }

//     pub fn ok_data(data: Value) -> Self {
//         Response{
//             ok: true,
//             message: None,
//             data: Some(data)
//         }
//     }

//     pub fn fail_none() -> Self {
//         Response {
//             ok: false,
//             message: None,
//             data: None,
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct ConfirmTaskArgs {
    pub account: AccountArg,
    pub timeout: Option<TQTimeout>,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TaskType {
    Enqueue,
    Dequeue,
    ApolloCheck,
    ApolloDemine,
    ApolloLogin,
    ApolloCreate,
    ApolloConfirm,
    ApolloScrape,
}

impl TaskType {
    pub async fn exec(
        &self,
        ctx: TaskActionCTX,
        args: Option<Value>,
    ) -> anyhow::Result<Option<Value>> {
        match self {
            // TaskType::ApolloCheck => apollo_check(ctx, args).await,
            // TaskType::ApolloDemine => apollo_demine(ctx, args).await,
            // TaskType::ApolloLogin => apollo_login(ctx, args).await,
            // TaskType::ApolloCreate => apollo_create(ctx, args).await,
            // TaskType::ApolloConfirm => apollo_confirm(ctx, args).await,
            // TaskType::ApolloScrape => apollo_scrape(ctx, args).await,
            _ => Ok(None),
        }
    }
}
