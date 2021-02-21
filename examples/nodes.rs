use nomad_client::model::nodes::NodeListStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:4646/v1/nodes")
        .await?
        .json::<Vec<NodeListStub>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
