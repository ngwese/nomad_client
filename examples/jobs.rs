use nomad_client::model::jobs::JobListStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:4646/v1/jobs")
        .await?
        .json::<Vec<JobListStub>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
