use std::cmp::{self, min};
use std::time::Duration;
use async_std::future::timeout;
use async_std::prelude::FutureExt;
use async_std::task::sleep;
use chromiumoxide::{Element, Page};
use fake::faker::internet::en::Username;
use fake::Fake;
use anyhow::{anyhow, Context, Result};
use serde_json::{from_value, json, Value};
use surrealdb::sql::{to_value, Id};
use tauri::{AppHandle, Manager, State};
use crate::actions::apollo::lib::index::{apollo_login_credits_info, log_into_apollo, log_into_apollo_then_visit};
use crate::actions::apollo::lib::util::{get_browser_cookies, get_page_in_url, goto_wait_for_selector, inject_cookies, set_page_in_url, set_range_in_url, time_ms, wait_for_selector, CreditsInfo};
use crate::actions::controllers::TaskType;
use crate::libs::cache::ApolloCache;
use crate::libs::db::accounts::types::{Account, History};
use crate::libs::db::metadata::types::{Accounts, Metadata, Scrapes};
use crate::libs::db::records::types::RecordDataArg;
use crate::libs::taskqueue::index::TaskQueue;
use crate::{
    actions::controllers::Response as R,
    libs::{
        db::index::DB,
        taskqueue::types::{Task, TaskActionCTX, TaskGroup},
    },
    SCRAPER,
};

use super::types::{PreviousLead, ScrapeActionArgs, ScrapeTaskArgs, MAX_LEADS_ON_PAGE};


#[tauri::command]
pub async fn scrape_task(ctx: AppHandle, args: Value) -> R<()> {
    // (FIX) should create meta_id in backend 
    let meta_id = match args.get("meta_id") {
        Some(val) => Some(val.clone()),
        None => None,
    };

    let fmt_args: ScrapeTaskArgs = from_value(args.clone()).unwrap();
    let metadata = match init_meta(&ctx.state::<DB>(), &fmt_args).await {
      Ok(meta) => meta,
      Err(_) => return R::fail_none(Some("Failed to scrape, could not initialize metadata"))
    };

    let cache = ctx.state::<ApolloCache>();

    for acc in from_value::<Vec<Accounts>>(args.get("accounts").unwrap().to_owned()).unwrap().iter() {
      let args = json!({
        "url": &fmt_args.url,
        "chunk": acc.chunk,
        "account_id": &acc.account_id,
        "metadata": &metadata,
        "max_leads_limit": &fmt_args.max_leads_limit
      });

      cache.add_accounts(meta_id.as_ref().unwrap().as_str().unwrap(), &mut vec![acc.account_id.clone()]).await;

      ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Id::uuid().to_string(),
        task_type: TaskType::ApolloScrape,
        task_group: TaskGroup::Apollo,
        message: "Scraping",
        metadata: Some(json!({ "meta_id": &meta_id })),
        timeout: args.get("timeout").and_then(|v| from_value(v.clone()).ok()),
        args: Some(args),
      });
    }

    R::ok_none()
}

pub async fn apollo_scrape(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>> {
    let mut args: ScrapeActionArgs = from_value(args.unwrap())?;
    ctx.page =  Some(unsafe { SCRAPER.incog().await? });
    let page = ctx.page.as_ref().unwrap();
    let db = ctx.handle.state::<DB>();
    
    let Some(mut account) = db.select_one::<Account>(
      "account",
      &args.account_id
    )
    .await? 
    else {
      return Err(anyhow!("Failed to scrape, could not find registered account"))
    };

    let mut url = set_range_in_url(&args.url, args.chunk);
    url = set_page_in_url(&args.url, 1);

    let apollo_max_page = if account.domain.contains("gmail") || 
    account.domain.contains("hotmail") ||
    account.domain.contains("outlook") { 3 } else { 5 };

    let mut prev_lead = PreviousLead {
      name: "".to_string(),
    };

    log_into_apollo_visit_credits(&ctx, &account).await?;
    let mut old_credits = apollo_login_credits_info(&ctx).await?;

    while args.max_leads_limit > 0 {
      let credits_left = old_credits.credit_limit - old_credits.credits_used;
      if credits_left <= 0 { return Ok(None) }

      let mut num_leads_to_scrape = cmp::min(args.max_leads_limit, credits_left.into());
      num_leads_to_scrape = cmp::min(num_leads_to_scrape, MAX_LEADS_ON_PAGE.into());

      if num_leads_to_scrape <= 0 { return Ok(None) }

      let scrape_id = Id::uuid().to_string();
      let list_name = Username().fake::<String>();

      let _ = update_db_for_new_scrape(&ctx, &mut args.metadata, &mut account, &list_name, &scrape_id).await?;

      let _ = go_to_search_url(page, &url).await?;
      let data = add_leads_to_list_and_scrape(
        &ctx, 
        &num_leads_to_scrape, 
        &list_name,
        &mut prev_lead
      ).await?;

      if data.len() == 0 { return Ok(None) };

      // sleep(Duration::from_secs(5)).await;

      let _ = page.goto("https://app.apollo.io/#/settings/credits/current").timeout(Duration::from_secs(5)).await;
      let new_credits = apollo_login_credits_info(&ctx).await?;

      let cookies = get_browser_cookies(&page).await?;
      
      let mut scrape = args.metadata.scrapes.iter()
        .find(|v| v.scrape_id == scrape_id)
        .ok_or("Failed to find scrape data").unwrap()
        .to_owned();
        
    
      let total_page_scrape = data.len() as u16;
      scrape.length = total_page_scrape as u8;

      let mut history = account.history.iter_mut()
        .find(|v| v.scrape_id == scrape_id)
        .ok_or("Failed to find history data").unwrap()
        .to_owned();

      account.total_scraped_recently = account.total_scraped_recently + total_page_scrape;
      history.total_page_scrape = total_page_scrape.clone();

      let (acc, metadata) = save_scrape_to_db(
        &ctx,
        &account._id,
        &args.metadata,
        &new_credits,
        &cookies,
        data,
        &scrape_id,
        scrape,
        history
      ).await?;

      let mut next_page: u8 = get_page_in_url(&url).unwrap() + 1;
      next_page = if next_page > apollo_max_page { 1 } else { next_page };
      url = set_page_in_url(&url, next_page);
      account = acc;
      args.metadata = metadata;
      
      args.max_leads_limit = args.max_leads_limit - total_page_scrape as u64;
      old_credits = new_credits;
    }

    Ok(None)
}

async fn update_db_for_new_scrape<'a, 'b>(ctx: &TaskActionCTX, metadata: &'a mut Metadata, account: &'b mut Account, list_name: &str, scrape_id: &str) -> Result<()> {
  let scrapes = Scrapes {
    scrape_id: scrape_id.to_string(), 
    list_name: list_name.to_string(), 
    length: 0, 
    date: "0".to_string()
  };

  let history = History {
    total_page_scrape: 0, 
    scrape_time: time_ms().to_string(), 
    list_name: list_name.to_string(),
    scrape_id: scrape_id.to_string()
  };
  
  let db_state = ctx.handle.state::<DB>();
  db_state.0.lock()
  .await
  .query(format!("
    BEGIN TRANSACTION;
    UPDATE metadata:{} SET scrapes += {};
    UPDATE account:{} SET history += {};
    COMMIT TRANSACTION;
  ",
    metadata._id,
    to_value(&scrapes)?,
    account._id,
    to_value(history)?,
  ))
  .await?;

  match db_state.select_one::<Metadata>("metadata", &metadata._id).await? {
    Some(meta) => { *metadata = meta; },
    None => { return Err(anyhow!("Failed to get metadata")) }
  };

  match db_state.select_one::<Account>("account", &account._id).await? {
    Some(acc) => { *account = acc; },
    None => { return Err(anyhow!("Failed to get account")) }
  };

  Ok(())
}


async fn go_to_search_url(page: &Page, url: &str) -> Result<()> {
  // page.goto("http://www.google.com").await;
  let current_url = page.url().await?.unwrap();
  let _ = page.goto(url).timeout(Duration::from_secs(5)).await;

  let _ = timeout::<_, Result<()>>(Duration::from_secs(30), async {
    loop {
      sleep(Duration::from_secs(5)).await;
      let new_url = page.url().await.unwrap().unwrap();
      if new_url != current_url { break }
    };
    Ok(())
  }).await;
  

  println!("WE IN go_to_search_url");
  wait_for_selector(page, r#"[class="zp_RFed0"]"#, 10, 2).await?;
  Ok(())
}

async fn add_leads_to_list_and_scrape(ctx: &TaskActionCTX, num_leads_to_scrape: &u64, list_name: &str, prev_lead: &mut PreviousLead) -> Result<Vec<RecordDataArg>> {
  let table_rows_selector = r#"[class="zp_RFed0"]"#;
  let checkbox_selector = r#"[class="zp_fwjCX"]"#;
  let add_to_list_input_selector = r#"[class="Select-input "]"#;
  let save_list_button_selector = r#"[class="zp-button zp_zUY3r"][type="submit"]"#;
  // let sl_popup_selector = r#"[class="zp_lMRYw zp_yHIi8"]"#;
  let saved_list_table_row_selector = r#"[class="zp_cWbgJ"]"#;
  let pagination_info_selector = r#"[class="zp_VVYZh"]"#;
  let page = ctx.page.as_ref().unwrap();

  wait_for_selector(&page, pagination_info_selector, 10, 4).await?;

  let name = timeout::<_, Result<String>>(Duration::from_secs(30), async {
    let prev_name = prev_lead.get();
    loop {
      let name = match get_first_table_row_name(&page).await {
        Ok(el_txt) => {
          match el_txt {
            Some(txt) => txt,
            None => prev_name.to_string()
          }
        },
        Err(_) => prev_name.to_string()
      };
      if name != prev_name { return Ok(name) }
      sleep(Duration::from_secs(2)).await;
    }
  }).await??;

  println!("CHECKIN PREVLEAD");
  println!("{:#?}", prev_lead.get());

  // let mut name = "".to_string();
  // let mut should_continue = false;
  // let mut counter: u8 = 0;
  // while !should_continue && counter <= 15 {
  //   if counter == 15 { return Err(anyhow!("failed to find to get first table row element")) }
  //   name = match get_first_table_row_name(&page).await {
  //     Ok(el_txt) => {
  //       match el_txt {
  //         Some(txt) => txt,
  //         None => name
  //       }
  //     },
  //     Err(e) => name
  //   };
  //   if name != prev_lead.get() { should_continue = true; }
  //   sleep(Duration::from_secs(2)).await;
  //   counter += 1;
  // }

  prev_lead.set(name);

  let rows = page.find_elements(table_rows_selector).await?;

  let max_leads = min(*num_leads_to_scrape, rows.len() as u64);

  for row_idx in 0..max_leads {
    if let Some(el) = rows.get(row_idx as usize) {
      el.find_element(checkbox_selector).await?.focus().await?.click().await?;
    }
  }

  let list_button = page.find_elements(r#"[class="zp-button zp_zUY3r zp_hLUWg zp_n9QPr zp_B5hnZ zp_MCSwB zp_ML2Jn"]"#).await?;
  if list_button.len() == 0 { return Err(anyhow!("failed to find list button")) }
  list_button[1].focus().await?.click().await?;

  let _list_button_2 = wait_for_selector(&page, r#"[class="zp-menu-item zp_fZtsJ zp_pEvFx"]"#, 10, 2).await?.focus().await?.click().await?;
  
  let mut counter = 0;
  while counter <= 5 {
    if counter == 5 { return Err(anyhow!("failed to find save list input element")) }
    let list_input = page.find_elements(add_to_list_input_selector).await?;
    if list_input.len() > 1 {
      list_input[1].focus().await?.type_str(&list_name).await?;
      break;
    }
    sleep(Duration::from_secs(3)).await;
    counter += 1;
  } 

  let _save_list_btn = page.find_element(save_list_button_selector).await?.focus().await?.click().await?;

  // wait_for_selector(&page, &sl_popup_selector, 10, 2).await;
  sleep(Duration::from_secs(5)).await;

  // =================================================================================================
  let _ = goto_wait_for_selector(
    page, 
    "https://app.apollo.io/#/people/tags?teamListsOnly[]=no", 
    &saved_list_table_row_selector, 
    15, 
    3
  ).await?;

  let mut counter = 0;
  while counter <= 10 {
    if counter == 10 { return Err(anyhow!("failed to find save list item")) };

    match wait_for_selector(page, saved_list_table_row_selector, 3, 2).await {
      Ok(el) => {
          match el.find_element(r#"[class="zp_aBhrx"]"#).await {
            Ok(el) => {
              if list_name == el.inner_text().await?.unwrap() {
                break;
              };
            },
            Err(e) => {
              println!("LOOP 2 {e:#?}");
            }
          }
      },
      Err(e) => {
        println!("LOOP 1 {e:#?}");
      }
    }

    page.reload().await?;
    sleep(Duration::from_secs(5)).await;
    counter += 1;
  }

  let _table_row = page.find_element(saved_list_table_row_selector).await?.focus().await?.click().await?;

  wait_for_selector(&page, &table_rows_selector, 10, 2).await?;

  scrape_leads(ctx).await
}

async fn scrape_leads(ctx: &TaskActionCTX) -> Result<Vec<RecordDataArg>> {
  ctx.page.as_ref().unwrap().evaluate_function(
  r#"
    async () => {
      window.na = "N/A";
      window.el = {
        columns: {
          name: {
            name: (columnEl) => isEl(columnEl.querySelector(".zp_xVJ20 > a")).innerHTML || na,
            linkedin: (columnEl) => isEl(columnEl.getElementsByClassName("zp-link zp_OotKe")[0]).href || na
          },
          title: (columnEl) => isEl(columnEl.querySelector(".zp_Y6y8d")).innerHTML || na,
          company: {
            name: (columnEl) => isEl(columnEl.getElementsByClassName("zp_WM8e5 zp_kTaD7")[0]).innerHTML || na,
            socialsList: (columnEl) => columnEl.getElementsByClassName("zp-link zp_OotKe") || []
          },
          location: (columnEl) => isEl(columnEl.querySelector(".zp_Y6y8d")).innerHTML || na,
          employees: (columnEl) => isEl(columnEl.querySelector(".zp_Y6y8d")).innerHTML || na,
          email: {
            emailButton: (columnEl) => columnEl.getElementsByClassName("zp-button zp_zUY3r zp_jSaSY zp_MCSwB zp_IYteB")[0],
            emailText: (columnEl) => isEl(columnEl.getElementsByClassName("zp-link zp_OotKe zp_Iu6Pf")[0]).innerHTML || na,
            noEmailText: (columnEl) => isEl(columnEl.getElementsByClassName("zp_RIH0H zp_Iu6Pf")[0]).innerHTML || na
          },
          industry: (columnEl) => isEl(columnEl.getElementsByClassName("zp_PHqgZ zp_TNdhR")[0]).innerHTML || na,
          keywords: (columnEl) => columnEl.getElementsByClassName("zp_yc3J_ zp_FY2eJ")
        },
        nav: {
          nextPageButton: () => document.getElementsByClassName("zp-button zp_zUY3r zp_MCSwB zp_xCVC8")[2],
          prevPageButton: () => document.getElementsByClassName("zp-button zp_zUY3r zp_MCSwB zp_xCVC8")[1],
          toggleFilterVisibility: () => document.getElementsByClassName("zp-button zp_zUY3r zp_MCSwB zp_xCVC8")[0]
        },
        ad: {
          isAdRow: (trEl) => isEl(trEl).className === "zp_DNo9Q zp_Ub5ME"
        },
        errors: {
          freePlan: {
            error: () => document.getElementsByClassName("zp_lMRYw zp_YYCg6 zp_iGbgU")[0],
            closeButton: () => document.getElementsByClassName(
              "zp-icon mdi mdi-close zp_dZ0gM zp_foWXB zp_j49HX zp_c5Xci"
            )[0].click()
          },
          limitedVersionError: () => document.getElementsByClassName(
            "apolloio-css-vars-reset zp zp-modal zp_iDDtd zp_APRN8 api-error-modal"
          )[0]
        }
      };
      window.isEl = (el2) => el2 ? el2 : {};
      window.scrapeSingleRow = async (tbody) => {
        const res = {};
        const columnNames = document.querySelectorAll("th");
        let tr = tbody.childNodes[0];
        if (el.ad.isAdRow(tr)) {
          tr = tbody.childNodes[1];
        }
        for (let i = 0; i < columnNames.length; i++) {
          switch (columnNames[i].innerText) {
            case "Name":
              const nameCol = scrapeNameColumn(tr.childNodes[i]);
              res["name"] = nameCol.name;
              res["firstname"] = nameCol.name.trim().split(" ")[0];
              res["lastname"] = nameCol.name.trim().split(" ")[1];
              res["linkedin"] = nameCol.linkedin;
              break;
            case "Title":
              res["title"] = scrapeTitleColumn(tr.childNodes[i]);
              break;
            case "Company":
              const companyCol = scrapeCompanyColumn(tr.childNodes[i]);
              res["company_name"] = companyCol.companyName;
              res["company_website"] = companyCol.companyWebsite;
              res["company_linkedin"] = companyCol.companyLinkedin;
              res["company_twitter"] = companyCol.companyTwitter;
              res["company_facebook"] = companyCol.companyFacebook;
              break;
            case "Quick Actions":
              res["email_1"] = await scrapeActionColumn(tr.childNodes[i]);
              break;
            case "Contact Location":
              res["company_location"] = scrapeLocationColumn(tr.childNodes[i]);
              break;
            case "Employees":
              res["employees"] = scrapeEmployeesColumn(tr.childNodes[i]);
              break;
            case "Phone":
              res["phone"] = scrapePhoneColumn(tr.childNodes[i]);
              break;
            case "Industry":
              res["industry"] = scrapeIndustryColumn(tr.childNodes[i]);
              break;
            case "Keywords":
              res["keywords"] = scrapeKeywordsColumn(tr.childNodes[i]);
          }
        }
        return res;
      };
      window.scrapeSinglePage = async () => {
        const allRows = getRows();
        const data = [];
        for (const row of allRows) {
          const singleRow = await scrapeSingleRow(row);
          if (singleRow)
            data.push(singleRow);
        }
        return data;
      };
      window.scrapeNameColumn = (nameColumn) => ({
        name: el.columns.name.name(nameColumn),
        linkedin: el.columns.name.linkedin(nameColumn)
      });
      window.scrapeTitleColumn = (titleColumn) => el.columns.title(titleColumn);
      window.scrapeCompanyColumn = (companyColumn) => {
        return {
          companyName: el.columns.company.name(companyColumn) || na,
          companyWebsite: na,
          companyLinkedin: na,
          companyTwitter: na,
          companyFacebook: na,
          ...Array.from(el.columns.company.socialsList(companyColumn)).reduce(
            (a, c) => ({
              ...a,
              // @ts-ignore
              ...populateSocialsLinks(c.href)
            }),
            {}
          )
        };
      };
      window.scrapeLocationColumn = (locationColumn) => el.columns.location(locationColumn);
      window.scrapeEmployeesColumn = (employeesColumn) => el.columns.employees(employeesColumn);
      window.scrapePhoneColumn = (phoneColumn) => phoneColumn.innerText === "Request Mobile Number" ? na : phoneColumn.innerText;
      window.scrapePhoneColumn = (phoneColumn) => phoneColumn.innerText === "Request Mobile Number" ? na : phoneColumn.innerText;
      window.scrapeEmailColumn = async (emailColumn) => {
        let loopCounter = 0;
        const loopEnd = 10;
        let email = "";
        const emailButton = emailColumn.getElementsByClassName(
          "zp-button zp_zUY3r zp_jSaSY zp_MCSwB zp_IYteB"
        )[0];
        let emailText = emailColumn.getElementsByClassName("zp-link zp_OotKe zp_Iu6Pf")[0];
        let noEmailText = emailColumn.getElementsByClassName("zp_RIH0H zp_Iu6Pf")[0];
        if (emailButton) {
          emailButton.click();
          while (!noEmailText && !emailText && loopCounter < loopEnd) {
            await sleep(15e3);
            loopCounter++;
            emailText = emailColumn.getElementsByClassName("zp-link zp_OotKe zp_Iu6Pf")[0];
            noEmailText = emailColumn.getElementsByClassName("zp_RIH0H zp_Iu6Pf")[0];
          }
          if (emailText) {
            email = emailText.innerHTML;
          }
        } else if (emailText) {
          email = emailText.innerHTML;
        }
        return email ? email : na;
      };
      window.scrapeActionColumn = async (emailColumn) => {
        let loopCounter = 0;
        const loopEnd = 3;
        let email = "";
        const emailButton = emailColumn.querySelector(
          '[class="zp-button zp_zUY3r zp_n9QPr zp_MCSwB"]'
        );
        let emailPopupButton = emailColumn.querySelector(
          '[class="zp-button zp_zUY3r zp_hLUWg zp_n9QPr zp_B5hnZ zp_MCSwB zp_IYteB"]'
        );
        let noEmailButton = emailColumn.querySelector(
          '[class="zp-button zp_zUY3r zp_BAp0M zp_jSaSY zp_MCSwB zp_IYteB zp_wUX4E zp_wUX4E"]'
        );
        if (emailButton) {
          emailButton.click();
          while (!emailPopupButton && !noEmailButton && loopCounter < loopEnd) {
            await sleep(5e3);
            loopCounter++;
            emailPopupButton = emailColumn.querySelector(
              '[class="zp-button zp_zUY3r zp_hLUWg zp_n9QPr zp_B5hnZ zp_MCSwB zp_IYteB"]'
            );
            noEmailButton = emailColumn.querySelector(
              '[class="zp-button zp_zUY3r zp_BAp0M zp_jSaSY zp_MCSwB zp_IYteB zp_wUX4E zp_wUX4E"]'
            );
            if (emailPopupButton) {
              emailPopupButton.click();
              email = await emailPopupButtonClick(emailPopupButton);
            } else if (noEmailButton) {
              email = na;
            }
          }
        } else if (emailPopupButton) {
          email = await emailPopupButtonClick(emailPopupButton);
        } else if (noEmailButton) {
          email = na;
        }
        return email ? email : na;
      };
      window.emailPopupButtonClick = async (emailPopupButton) => {
        let email = "-";
        let loopCounter = 0;
        const loopEnd = 5;
        let emailText = document.getElementsByClassName("zp_t08Bv")[0];
        emailPopupButton.click();
        if (!emailText) {
          while (!emailText && loopCounter < loopEnd) {
            await sleep(1e3);
            loopCounter++;
            emailText = document.getElementsByClassName("zp_t08Bv")[0];
            if (emailText) {
              email = emailText.innerHTML;
            }
          }
        } else if (emailText) {
          email = emailText.innerHTML;
        }
        emailPopupButton.click();
        return email;
      };
      window.scrapeIndustryColumn = (industryColumn) => el.columns.industry(industryColumn);
      window.scrapeKeywordsColumn = (keywordsColumn) => {
        return Array.from(el.columns.keywords(keywordsColumn)).reduce(
          (a, cv) => a += cv.innerHTML,
          ""
        );
      };
      window.populateSocialsLinks = (companyLink) => {
        const data = {};
        const lowerCompanyLink = companyLink.toLowerCase();
        if (!lowerCompanyLink.includes("linkedin.") && !lowerCompanyLink.includes("twitter.") && !lowerCompanyLink.includes("facebook.")) {
          data["companyWebsite"] = companyLink;
        } else if (lowerCompanyLink.includes("linkedin.")) {
          data["companyLinkedin"] = companyLink;
        } else if (lowerCompanyLink.includes("twitter.")) {
          data["companyTwitter"] = companyLink;
        } else if (lowerCompanyLink.includes("facebook.")) {
          data["companyFacebook"] = companyLink;
        }
        return data;
      };
      window.sleep = (ms) => {
        return new Promise((resolve) => setTimeout(resolve, ms));
      };
      window.getRows = () => {
        return document.getElementsByClassName("zp_RFed0");
      };
      return await scrapeSinglePage();
    }
  "#,
  )
  .await?.into_value::<Vec<RecordDataArg>>().context("Failed To collect and parce leads")
}


async fn get_first_table_row_name(page: &Page) -> Result<Option<String>> {
  let row = page.find_element(r#"[class="zp_BC5Bd"]"#).await?;
  // row = row.find_element(r#"[class="zp_BC5Bd"]"#).await?;
  Ok(row.inner_text().await?)
}

async fn save_scrape_to_db(
  ctx: &TaskActionCTX, 
  account_id: &str, 
  metadata: &Metadata, 
  credits: &CreditsInfo, 
  cookies: &str, 
  data: Vec<RecordDataArg>, 
  scrape_id: &str, 
  scrape: Scrapes,
  history: History
) -> Result<(Account, Metadata)> {
  let mut query = vec![
    "BEGIN TRANSACTION;".to_string(),
    format!("UPDATE account:{} MERGE $accountdata;", account_id),
    format!("UPDATE account:{} SET history[WHERE scrape_id=\"{}\"] = {};", account_id, scrape_id, to_value(history).unwrap()),
    format!("UPDATE metadata:{} SET scrapes[WHERE scrape_id=\"{}\"] = {};", &metadata._id, scrape_id, to_value(scrape).unwrap())
  ];
  for d in data {
    query.push(
        format!(
          "CREATE record:{} CONTENT {};", 
          Id::rand(),
          to_value(json!({
            "scrape_id": scrape_id,
            "url": &metadata.url,
            "data": &d
          }))?
        
      )
    );
  }
  query.push("COMMIT TRANSACTION;".to_string());
  
  let db_state = ctx.handle.state::<DB>();
  let db_guard = db_state.0.lock().await;
  let _ = db_guard
    .query(query.join(" "))
    .bind(("accountdata", json!({
        "cookies": cookies,
        "last_used": time_ms().to_string(),
        "credits_used": credits.credits_used,
        "credit_limit": credits.credit_limit,
        "renewal_date": credits.renewal_date,
        "renewal_start_date": credits.renewal_start_date,
        "renewal_end_date": credits.renewal_end_date,
        "trial_days_left": credits.trial_days_left.as_ref().or(None)
        }))
    ).await?;

    drop(db_guard);

    let metadata = match db_state.select_one::<Metadata>("metadata", &metadata._id).await? {
      Some(meta) => meta,
      None => { return Err(anyhow!("Failed to get metadata")) }
    };
  
    let account = match db_state.select_one::<Account>("account", account_id).await? {
      Some(acc) => acc,
      None => { return Err(anyhow!("Failed to get account")) }
    };

    Ok((account, metadata))
    // -> Result<(Account, Metadata)>
}

async fn init_meta<'a>(db: &State<'a, DB>, args: &ScrapeTaskArgs) -> Result<Metadata> {
  match db.select_one::<Metadata>(
    "metadata",
    &args.meta_id
  ).await {
    Ok(meta) => {
      if meta.is_none() {
        return new_meta(db, args).await;
      }
      Ok(meta.unwrap())
    },
    Err(_) => {
      return Err(anyhow!("Failed to scrape, could not find or create metadata"))
    }
  }
}

async fn new_meta<'a>(db: &State<'a, DB>, args: &ScrapeTaskArgs) -> Result<Metadata> {
  let Some(meta) = db.insert_one::<Metadata>(
    "metadata",
    &args.meta_id,
    to_value(
      Metadata {
        _id: args.meta_id.clone(),
        url: args.url.clone(),
        // params: args.params.clone(),
        name: args.name.clone(),
        scrapes: vec![],
        accounts: args.accounts.clone()
      }
    )?
  ).await? else {
    return Err(anyhow!("Failed to scrape, could not register metadata"))
  };

  Ok(meta)
}

pub async fn log_into_apollo_visit_credits(
  ctx: &TaskActionCTX,
  account: &Account,
) -> Result<()> {
  let login_input_field_selector = "input[class=\"p_bWS5y zp_J0MYa\"]".to_string();
  let credits_component_selector = "[class=\"zp-card zp_T6GzV zp_kUStV zp_umSnZ zp_VicPq\"]".to_string();
  let page = ctx.page.as_ref().unwrap();

  inject_cookies(&page, &account.cookies).await?;

  let _ = page.goto("https://app.apollo.io/#/settings/credits/current")
      .await?
      .wait_for_navigation()
      .timeout(Duration::from_secs(60))
      .await;

  timeout::<_, Result<()>>(Duration::from_secs(60), async {
    loop {
      let login_component: Option<Element> = page.find_element(&login_input_field_selector).await.ok();
      let credits_component: Option<Element> = page.find_element(&credits_component_selector).await.ok();
      
      if login_component.is_some() {
        log_into_apollo(ctx, account).await;
        return Ok(());
      }

      if credits_component.is_some() {
        return Ok(());
      }

      sleep(Duration::from_secs(5)).await;
    }
  }).await?;

  ctx.handle
      .emit_all(
          "apollo",
          json!({
              "task_id": ctx.task_id, 
              "message": "Logged into apollo"
          }),
      )
      .unwrap();

  Ok(())
}