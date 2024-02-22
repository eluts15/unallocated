use aws_sdk_route53::types::ResourceRecordSet;
use aws_sdk_route53::{Client, Error};

// TODO: Document fn
pub async fn search_hosted_zones(client: &Client) -> Result<String, Error> {
    let hosted_zones = client.list_hosted_zones().send().await?;

    // TODO: Support many hosted zones.
    let zone_id = hosted_zones
        .hosted_zones()
        .first()
        .map(|zone| zone.id().to_string())
        .expect("multi-zones isn't supported...yet.");

    Ok(zone_id)
}

pub async fn fetch_all_resource_record_sets(
    client: &Client,
    hosted_zone_id: &str,
) -> Result<Vec<ResourceRecordSet>, Error> {
    let hosted_zone_id = &hosted_zone_id;

    if hosted_zone_id.is_empty() {
        println!("Zone Error: {:?}\n", hosted_zone_id);
    } else {
        println!("Zone ID found, listing records in: {:?}\n", hosted_zone_id);
    }

    // dumb af
    let response = client
        .list_resource_record_sets()
        .max_items(100)
        .hosted_zone_id(hosted_zone_id.to_owned())
        .send()
        .await?;

    let mut next_record_name = response.next_record_name;
    let mut all_record_sets = Vec::new();

    loop {
        let response = client
            .list_resource_record_sets()
            .hosted_zone_id(hosted_zone_id.to_owned())
            .start_record_name(next_record_name.unwrap_or_default())
            .start_record_type("A".into())
            .send()
            .await?;
        // Append the record sets to the vector
        all_record_sets.extend(response.resource_record_sets);

        // Check if there are more record sets available
        if response.is_truncated {
            next_record_name = response.next_record_name;
        } else {
            // No more record sets available, break out of the loop
            break;
        }
    }
    Ok(all_record_sets)
}
