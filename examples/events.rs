use bytes::Bytes;
use futures_util::StreamExt;
use nomad_client::model::event_stream::Events;
use nomad_client::chunked_response::Assembler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("http://127.0.0.1:4646/v1/event/stream")
        .await?;

    // check to ensure the payload is utf-8? application/json is utf-8
    let headers = response.headers();
    println!("Headers: {:?}", headers);

    let mut stream = response.bytes_stream();
    let mut assembler = Assembler::new();

    let newline = Bytes::from("\n");

    while let Some(item) = stream.next().await {
        match item {
            Ok(something) => {
                if something == newline {
                    continue;
                }

                let decoded = std::str::from_utf8(&something);
                match decoded {
                    Err(e) => {
                        println!("Decoding error: {}", e);
                        continue;
                    },
                    Ok(text) => {
                        if let Some(events) = assembler.add::<Events>(&text) {
                            println!("{:#?}\n---", events);
                        }
                    }
                }
            },
            Err(e) => {
                println!("Stream Error: {:?}", e);
            }
        }
    }

    Ok(())
}
