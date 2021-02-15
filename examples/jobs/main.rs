use nomad_client::jobs::JobListStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:4646/v1/jobs")
        .await?
        .text()
        .await?;
    let jobs: Vec<JobListStub> = serde_json::from_str(resp.as_str())?;
    println!("{:#?}", jobs);
    Ok(())
}
