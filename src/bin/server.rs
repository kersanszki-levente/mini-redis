use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};

use myredis::command::Command::{self, Get, Set};
use myredis::connection::Connection;
use myredis::frame::Frame;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(ShardedDB::new(12));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        }).await.unwrap();
    }
}

type Shard = Mutex<HashMap<String, Bytes>>;

struct ShardedDB {
    shards: Vec<Shard>
}

impl ShardedDB {
    fn new(n_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(n_shards);
        for _ in 0..n_shards {
            shards.push(Mutex::new(HashMap::new()));
        }
        Self { shards }
    }
    fn insert(&self, key: &str, value: &Bytes) {
        let hash = 0; // TO-DO: Add proper hashing
        let shard = hash % self.shards.len();
        self.shards[shard]
            .lock()
            .unwrap()
            .insert(key.to_string(), value.clone());
    }
    fn get(&self, key: &str) -> Option<bytes::Bytes> {
        let hash = 0; // TO-DO: Add proper hashing
        let shard = hash % self.shards.len();
        self.shards[shard].lock().unwrap().get(key).cloned()
    }
}

async fn process(socket: TcpStream, db: Arc<ShardedDB>) {

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key(), cmd.value());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            // cmd => panic!("Not implemented: {cmd:?}"),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
