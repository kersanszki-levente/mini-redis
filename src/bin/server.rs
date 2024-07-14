use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

use myredis::command::Command::{self, Get, Set};
use myredis::connection::Connection;
use myredis::db::ShardedDB;
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
