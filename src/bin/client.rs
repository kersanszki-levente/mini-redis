use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("0.0.0.0:6379").await?;

    client.set("hello", "world".into()).await?;

    let response = client.get("hello").await?;

    println!("got value {response:?}");
    Ok(())
}
