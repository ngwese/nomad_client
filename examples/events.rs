use bytes::Bytes;
use futures_util::StreamExt;
use nomad_client::event_stream::Events;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = reqwest::get("http://127.0.0.1:4646/v1/event/stream?pretty")
        .await?
        .bytes_stream();

    let newline = Bytes::from("\n");
    while let Some(item) = stream.next().await {
        match item {
            Ok(something) => {
                if something != newline {
                    let chunk = serde_json::from_slice::<Events>(&something[..]);
                    match chunk {
                        Ok(events) => println!("{:#?}\n---", events),
                        Err(e) => println!("Parse Error: {:?}", e),
                    }
                }
            }
            Err(e) => {
                println!("Stream Error: {:?}", e);
            }
        }
    }

    Ok(())
}
