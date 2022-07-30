use std::collections::HashMap;

use reqwest::Client;

pub fn convert_to_string(val: &str) -> String {
    return String::from(val);
}

pub async fn post_request(
    url: &str,
    note_map: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = reqwest::Client::new();
    let resp = client.post(url).json(&note_map).send().await?;

    if resp.status() != 200 {
        println!("Failed to upload notes...");
        return Ok(());
    }

    return Ok(());
}
