use crate::service::request::structs::PythonReleaseData;
use anyhow::Result;
use reqwest::Client;

pub async fn get_python_versions(client: &Client) -> Result<Vec<PythonReleaseData>> {
    const VERSION_API_URL: &str = "https://endoflife.date/api/python.json";
    println!("Requesting versions..");
    let response = client
                    .get(VERSION_API_URL)
                    .header("User-Agent", "pancakes/0.1 (Educational tool for Python learning; more info: https://github.com/moltenlavaguava/pancakes)")
                    .send()
                    .await?;
    println!("got response: {:?}", response);
    Ok(serde_json::from_str(
        &response.error_for_status()?.text().await?,
    )?)
}
