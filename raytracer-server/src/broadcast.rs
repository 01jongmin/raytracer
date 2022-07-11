use std::collections::HashMap;

use actix_web::{
    web::{Bytes, Data},
    cookie::ParseError as Error
};

use std::sync::Mutex;
use tokio::sync::watch::{channel, Receiver, Sender};
use tokio_stream::wrappers::WatchStream;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

struct BroadcasterInner {
    clients: HashMap<u64, (Sender<Result<Bytes, Error>>, Receiver<Result<Bytes, Error>>)>,
}

impl Broadcaster {
    pub fn create() -> Data<Self> {
        let me = Data::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner {
                clients: HashMap::new(),
            }),
        });

        me
    }

    pub fn close_sender(&self, key: u64) {
        let mut inner = self.inner.lock().expect("crash");

        inner.clients.remove(&key);
    }

    pub fn get_number(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.clients.len()
    }

    pub fn new_connection(&self, key: u64) {
        let (tx, rx) = channel(Ok(Bytes::from("hello")));

        tx.send(Ok(Bytes::from("data: connected\n\n"))).unwrap();

        let mut inner = self.inner.lock().unwrap();
        inner.clients.insert(key, (tx, rx));
   }

   pub fn get_client(&self, key: u64) -> Option<WatchStream<Result<Bytes, Error>>> {
        let inner = self.inner.lock().unwrap();

        if let Some((_, receiver)) = inner.clients.get(&key) {
            Some(WatchStream::new(receiver.clone()))
        } else {
            None
        }
   }

    pub fn send(&self, key: u64, msg: &str) {
        println!("{}", msg);
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        let inner = self.inner.lock().unwrap();
        if let Some((sender, _)) = inner.clients.get(&key) {
            if let Err(_) = sender.send(Ok(msg.clone())) {
                println!("ERROR");
            }
        } else {
            println!("ERROR");
        }
    }
}
