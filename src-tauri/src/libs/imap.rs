use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_std::{
  channel::{unbounded, Receiver, Sender}, net::TcpStream, sync::Mutex, task::{sleep, spawn, JoinHandle}
};

use async_imap::{self, Session};
use async_native_tls::{TlsConnector, TlsStream};
use futures::TryStreamExt;
use mailparse::*;
use regex::Regex;

#[derive(Debug)]
pub struct IMAPEnvelope {
  pub subject: Option<String>,
  pub from: Option<String>,
  pub to: Option<String>,
  pub link_1: Option<String>,
  pub link_2: Option<String>
}

#[derive(Debug)]
pub struct IMAP {
  pub imap: Arc<Mutex<Option<Session<TlsStream<TcpStream>>>>>,
  pub poll: Mutex<Option<JoinHandle<()>>>,
  pub sender: Sender<IMAPEnvelope>,
  pub receiver: Receiver<IMAPEnvelope>,
  pub watching: Mutex<u8>
}

impl IMAP {
  pub fn new() -> Self {
    let (sender, receiver) = unbounded::<IMAPEnvelope>();
    
    Self {
      imap: Arc::new(Mutex::new(None)),
      sender,
      receiver,
      poll: Mutex::new(None),
      watching: Mutex::new(0)
    }
  }

  
  pub async fn connect(&self) -> Result<()> {
    let stream = TcpStream::connect(("imap.gmail.com", 993)).await?;
    let tls = TlsConnector::new().use_sni(true).connect("google.com", stream).await?;
    let client = async_imap::Client::new(tls);

    let mut session = client.login("miom", "ibgqfdskegtskvog")
    .await
    .map_err(|(err, _client)| err)?;

    session.select("INBOX").await?;

    let mut guard = self.imap.lock().await; 
    *guard = Some(session);
    drop(guard);
  
    let imap_sender = self.sender.clone();
    let imap_clone = self.imap.clone();
    let poll = spawn(async move {
      let mut imap_clone = imap_clone.lock().await;
      loop {
        sleep(Duration::from_secs(7)).await;
        let message_stream = imap_clone.as_mut().unwrap().fetch("1:10", "RFC822").await.unwrap();
        let messages: Vec<_> = message_stream.try_collect().await.unwrap();
        for message in messages.iter() {
          if let Some(body) = message.body() {
            let mail = parse_mail(body).unwrap();

            let subject = mail.headers.get_first_value("Subject");

            if subject.as_ref().unwrap().contains("Activate Your Apollo Account") {
              let body = mail.subparts[0].get_body().unwrap();
              let link1_regex = Regex::new(r"(?<link>https://app.tryapollo.io[\S|\n]+)").unwrap();
              let link2_regex = Regex::new(r"(?<link>https://app.tryapollo.io/#[\S|\n]+)").unwrap();

              let link_1 = link1_regex.captures(&body).and_then(|v| Some(v["link"].to_string()));
              let link_2 = link2_regex.captures(&body).and_then(|v| Some(v["link"].to_string()));

              imap_sender.send(IMAPEnvelope {
                subject,
                from: mail.headers.get_first_value("From"),
                to: mail.headers.get_first_value("To"),
                link_1,
                link_2
              }).await.unwrap();
            }
          }
        }
      }
    });

    let mut guard = self.poll.lock().await;
    *guard = Some(poll);

    Ok(())
  }

  pub async fn logout(&self) -> Result<()> {
    let mut imap = self.imap.lock().await;
    imap.as_mut().unwrap().logout().await?;
    *imap = None;
    Ok(())
  }

  pub async fn watch(&self) -> Receiver<IMAPEnvelope> {
    if self.imap.lock().await.is_none() {
      self.connect().await.unwrap();
    }
    let mut watching = self.watching.lock().await;
    *watching += 1;

    self.receiver.clone()
  }

  pub async fn unwatch(&self) {
    let mut watching = self.watching.lock().await;

    if *watching == 1 {
      if self.imap.lock().await.is_some() {
        self.logout().await.unwrap();
      }
      *watching = 0;
    } else if *watching > 1 {
      *watching -= 1 
    }
  }

}