use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub async fn list_linode_instances(token: String) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = "https://api.linode.com/v4/linode/instances";

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    let json_response: Value = serde_json::from_str(&response)?;

    //println!("{}", json_response);
    Ok(json_response)
}

pub async fn extract(
    json_response: &str,
) -> Result<Vec<(u64, String, String)>, Box<dyn std::error::Error>> {
    let json_response: Value = serde_json::from_str(json_response)?;

    let data = match json_response.get("data") {
        Some(Value::Array(d)) => d,
        _ => return Err("No field 'data' found".into()),
    };

    let mut extracted_data = Vec::new();

    for entry in data {
        let id = match entry["id"].as_u64() {
            Some(id) => id,
            None => return Err("No field 'id' found".into()),
        };

        let label = match entry["label"].as_str() {
            Some(label) => label.to_string(),
            None => return Err("No field 'label' found.".into()),
        };

        let ipv4 = match entry["ipv4"][0].as_str() {
            Some(ipv4) => ipv4.to_string(),
            None => return Err("No field 'ipv4' found.".into()),
        };

        extracted_data.push((id, label, ipv4));
    }

    Ok(extracted_data)
}
