use nomad_client::nodes::NodeListStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:4646/v1/nodes")
        .await?
        .text()
        .await?;
    let nodes: Vec<NodeListStub> = serde_json::from_str(resp.as_str())?;
    println!("{:#?}", nodes);
    Ok(())
}
