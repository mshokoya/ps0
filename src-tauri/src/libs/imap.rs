use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_std::{
  channel::{unbounded, Receiver, Sender}, net::TcpStream, sync::Mutex, task::{sleep, spawn, JoinHandle}
};

use async_imap::{self, Session};
use async_native_tls::{TlsConnector, TlsStream};
use futures::TryStreamExt;
use imap_proto::Address;

#[derive(Debug)]
pub struct IMAPEnvelope {
  pub date: Option<String>,
  pub subject: Option<String>,
  pub from: Option<Vec<IMAPAddress>>,
  pub sender: Option<Vec<IMAPAddress>>,
  pub to: Option<Vec<IMAPAddress>>,
  pub message_id: Option<String>,
  pub body: Option<String>,
}

#[derive(Debug)]
pub struct IMAPAddress {
  pub name: Option<String>,
  pub adl: Option<String>,
  pub mailbox: Option<String>,
  pub host: Option<String>,
}

#[derive(Debug)]
pub struct IMAP {
  pub imap: Arc<Mutex<Option<Session<TlsStream<TcpStream>>>>>,
  pub poll: Mutex<Option<JoinHandle<()>>>,
  pub sender: Sender<IMAPEnvelope>,
  pub receiver: Receiver<IMAPEnvelope>,
  pub watching: Mutex<u8>
}

// unsafe impl Send for IMAP {}
// unsafe impl Sync for IMAP {}


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

  // https://github.com/async-email/async-imap/tree/main/examples/src/bin
  pub async fn connect(&self) -> Result<()> {
    let stream = TcpStream::connect(("imap.google.com", 993)).await?;
    let tls = TlsConnector::new().use_sni(true).connect("imap.google.com", stream).await?;
    let client = async_imap::Client::new(tls);

    let mut session = client.login("mikeydee0161@gmail.com", "ibgqnrpzfdsdxskvog")
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
        println!("PRINT DA POOL NIGGA");
        sleep(Duration::from_secs(7)).await;
        // https://blog.logrocket.com/email-crates-for-rust-lettre-and-imap/
        let message_stream = imap_clone.as_mut().unwrap().fetch("1:10", "RFC822").await.unwrap();
        let messages: Vec<_> = message_stream.try_collect().await.unwrap();
        for message in messages.iter() {
          let envelope = message.envelope();
          imap_sender.send(IMAPEnvelope {
            date: envelope.and_then(|e| extract_frm_envelope(e.date.as_deref())),
            subject: envelope.and_then(|e| extract_frm_envelope(e.subject.as_deref())),
            from: envelope.and_then(|e| extract_addrs_envelope(&e.from)),
            sender: envelope.and_then(|e| extract_addrs_envelope(&e.sender)),
            to: envelope.and_then(|e| extract_addrs_envelope(&e.to)),
            message_id: envelope.and_then(|e| extract_frm_envelope(e.message_id.as_deref())),
            body: extract_frm_envelope(message.body()),
          }).await.unwrap();
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


fn extract_frm_envelope(env: Option<&[u8]>) -> Option<String> {
  env.and_then(|d| std::str::from_utf8(d).ok().and_then(|s| Some(s.to_string())))
}

fn extract_addrs_envelope(env: &Option<Vec<Address>>) -> Option<Vec<IMAPAddress>> {
  env.as_ref().and_then(|d| {
    Some(
      d.iter().map(|add| {
        // Cow<&[u8]>
        IMAPAddress { 
          name: extract_frm_envelope(add.name.as_deref()),
          adl: extract_frm_envelope(add.adl.as_deref()),
          mailbox: extract_frm_envelope(add.mailbox.as_deref()),
          host: extract_frm_envelope(add.host.as_deref()),
        }
      }).collect::<Vec<IMAPAddress>>()
    )
  })
}


// use std::{ net::TcpStream, sync::Arc, time::Duration};

// use anyhow::Result;
// use async_std::{
//   channel::{bounded, Receiver, Sender}, sync::Mutex, task::{sleep, spawn, JoinHandle}
// };
// use imap::{self, Session};
// use native_tls::TlsStream;
// use imap_proto::types::Address;

// #[derive(Debug)]
// pub struct IMAPEnvelope {
//   pub date: Option<String>,
//   pub subject: Option<String>,
//   pub from: Option<Vec<IMAPAddress>>,
//   pub sender: Option<Vec<IMAPAddress>>,
//   pub to: Option<Vec<IMAPAddress>>,
//   pub message_id: Option<String>,
//   pub body: Option<String>,
// }

// #[derive(Debug)]
// pub struct IMAPAddress {
//   pub name: Option<String>,
//   pub adl: Option<String>,
//   pub mailbox: Option<String>,
//   pub host: Option<String>,
// }

// #[derive(Debug)]
// pub struct IMAP {
//   pub imap: Arc<Mutex<Option<Session<TlsStream<TcpStream>>>>>,
//   pub poll: Mutex<Option<JoinHandle<()>>>,
//   pub sender: Sender<IMAPEnvelope>,
//   pub receiver: Receiver<IMAPEnvelope>,
//   pub watching: Mutex<u8>
// }

// // unsafe impl Send for IMAP {}
// // unsafe impl Sync for IMAP {}


// impl IMAP {
//   pub fn new() -> Self {
//     let (sender, receiver) = bounded::<IMAPEnvelope>(0);
    
//     Self {
//       imap: Arc::new(Mutex::new(None)),
//       sender,
//       receiver,
//       poll: Mutex::new(None),
//       watching: Mutex::new(0)
//     }
//   }

  
//   pub async fn connect(&self) -> Result<()> {
//     let domain = "imap.gmail.com";
//     let tls = native_tls::TlsConnector::builder().build().unwrap();

//     let client = imap::connect((domain, 993), domain, &tls).unwrap();
    
//     let mut session = client
//       .login("mikeydee0161@gmail.com", "ibgqnrpzhgtskvog")
//       .map_err(|e| e.0)?;

//     session.select("INBOX").unwrap();

//     let mut guard = self.imap.lock().await; 
//     *guard = Some(session);
//     drop(guard);
  
//     let imap_sender = self.sender.clone();
//     let imap_clone = self.imap.clone();
//     let poll = spawn(async move {
//       let mut imap_clone = imap_clone.lock().await;
//       loop {
//         println!("PRINT DA POOL NIGGA");
//         sleep(Duration::from_secs(7)).await;
//         // https://blog.logrocket.com/email-crates-for-rust-lettre-and-imap/
//         let messages = imap_clone.as_mut().unwrap().fetch("1:10", "RFC822").unwrap();
        
//         for message in messages.iter() {
//           let envelope = message.envelope();
//           imap_sender.send(IMAPEnvelope {
//             date: envelope.and_then(|e| extract_frm_envelope(e.date)),
//             subject: envelope.and_then(|e| extract_frm_envelope(e.subject)),
//             from: envelope.and_then(|e| extract_addrs_envelope(&e.from)),
//             sender: envelope.and_then(|e| extract_addrs_envelope(&e.sender)),
//             to: envelope.and_then(|e| extract_addrs_envelope(&e.to)),
//             message_id: envelope.and_then(|e| extract_frm_envelope(e.message_id)),
//             body: extract_frm_envelope(message.body()),
//           }).await.unwrap();
//         }
//       }
//     });

//     let mut guard = self.poll.lock().await;
//     *guard = Some(poll);

//     Ok(())
//   }

//   pub async fn logout(&self) -> Result<()> {
//     let mut imap = self.imap.lock().await;
//     imap.as_mut().unwrap().logout()?;
//     *imap = None;
//     Ok(())
//   }

//   pub async fn watch(&self) -> Receiver<IMAPEnvelope> {
//     if self.imap.lock().await.is_none() {
//       self.connect().await.unwrap();
//     }
//     let mut watching = self.watching.lock().await;
//     *watching += 1;

//     self.receiver.clone()
//   }

//   pub async fn unwatch(&self) {
//     let mut watching = self.watching.lock().await;

//     if *watching == 1 {
//       if self.imap.lock().await.is_some() {
//         self.logout().await.unwrap();
//       }
//       *watching = 0;
//     } else if *watching > 1 {
//       *watching -= 1 
//     }
//   }

// }


// fn extract_frm_envelope(env: Option<&[u8]>) -> Option<String> {
//   env.and_then(|d| std::str::from_utf8(d).ok().and_then(|s| Some(s.to_string())))
// }

// fn extract_addrs_envelope(env: &Option<Vec<Address>>) -> Option<Vec<IMAPAddress>> {
//   env.as_ref().and_then(|d| {
//     Some(
//       d.iter().map(|add| {
//         // Cow<&[u8]>
//         IMAPAddress { 
//           name: extract_frm_envelope(add.name),
//           adl: extract_frm_envelope(add.adl),
//           mailbox: extract_frm_envelope(add.mailbox),
//           host: extract_frm_envelope(add.host),
//         }
//       }).collect::<Vec<IMAPAddress>>()
//     )
//   })
// }
