use aws_sdk_route53::{Client, Error};

// TODO: Document fn
pub async fn search_hosted_zones(client: &Client) -> Result<String, Error> {
    let hosted_zones = client.list_hosted_zones().send().await?;

    let mut zone_ids = String::new();
    for zone in hosted_zones.hosted_zones() {
        let _zone_name = zone.name();
        let zone_id = zone.id();

        zone_ids.push_str(zone_id);
    }

    Ok(zone_ids)
}

// TODO: Document fn
pub async fn list_all_resource_record_sets(
    client: &Client,
    hosted_zone_id: &str,
) -> Result<Vec<(String, Vec<String>, String)>, Error> {
    let hosted_zone_id = &hosted_zone_id;
    if hosted_zone_id.is_empty() {
        println!("Zone Error: {:?}\n", hosted_zone_id);
    } else {
        println!("Zone ID found, listing records in: {:?}\n", hosted_zone_id);
    }
    let response = client
        .list_resource_record_sets()
        .hosted_zone_id(hosted_zone_id.to_owned())
        .send()
        .await?;

    let resource_record_sets = response.resource_record_sets;
    let mut record_set_type_a_records = Vec::new(); // Create a Vector of only the "A" records.

    // Only get records of type "A".
    for record_set in resource_record_sets.iter() {
        if let Some(record_type) = record_set.r#type().into() {
            if record_type == &aws_sdk_route53::types::RrType::A {
                record_set_type_a_records.push(record_set);
            }
        } else {
            eprintln!("Error parsing record types.");
            // Handle the case where the record type is not present
        }
    }

    let a_records = record_set_type_a_records;

    // TODO: Look into this redunancy.
    let parsed_records: Vec<(&str, Vec<&str>, &str)> = a_records
        .iter()
        .map(|record_set| {
            let name = record_set.name.as_str();
            let resource_records: Vec<&str> = record_set
                .resource_records
                .as_ref()
                .map_or_else(Vec::new, |records| {
                    records.iter().map(|record| record.value.as_str()).collect()
                });
            let r#type = record_set.r#type.as_str();
            (name, resource_records, r#type)
        })
        .collect();

    // Cloning the data inside the tuple before returning
    let cloned_records: Vec<(String, Vec<String>, String)> = parsed_records
        .iter()
        .map(|(name, records, r#type)| {
            (
                name.to_string(),
                records.iter().map(|s| s.to_string()).collect(),
                r#type.to_string(),
            )
        })
        .collect();

    Ok(cloned_records)
}
