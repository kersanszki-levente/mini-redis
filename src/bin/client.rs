use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use mini_redis::{client, Result};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {

        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get{key: "foo".to_string(), resp: resp_tx};
        tx.send(cmd).await.unwrap();
        let response = resp_rx.await.unwrap();
        println!("GOT = {response:?}")

    });
    let t2 = tokio::spawn(async move {

        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {key: "foo".to_string(), value: "bar".into(), resp: resp_tx};
        tx2.send(cmd).await.unwrap();
        let response = resp_rx.await.unwrap();
        println!("GOT = {response:?}")

    });

    let manager = tokio::spawn(async move {

        let mut client = client::connect("0.0.0.0:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let response = client.get(&key).await;
                    let _ = resp.send(response).unwrap();
                },
                Command::Set { key, value, resp } => {
                    let response = client.set(&key, value.clone()).await;
                    let _ = resp.send(response).unwrap();
                }
            }
        }

    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

    Ok(())
}
