use aws_sdk_route53::{Client, Error};

pub async fn fetch_hosted_zones(client: &Client) -> Result<String, Error> {
    let hosted_zones = client.list_hosted_zones().send().await?;

    let mut zone_ids = String::new();
    for zone in hosted_zones.hosted_zones() {
        let _zone_name = zone.name();
        let zone_id = zone.id();

        zone_ids.push_str(zone_id);
    }

    Ok(zone_ids)
}

pub async fn list_all_resource_record_sets(
    client: &Client,
    hosted_zone_id: &str,
) -> Result<(), Error> {
    let hosted_zone_id = &hosted_zone_id;
    if hosted_zone_id.is_empty() {
        println!("Zone Error: {:?}", hosted_zone_id);
    } else {
        println!("Zone ID found: {:?}", hosted_zone_id);
    }

    println!("Attempting to fetch records...");
    println!("{:?}", hosted_zone_id.to_string());

    let response = client
        .list_resource_record_sets()
        .hosted_zone_id(hosted_zone_id.to_owned())
        .send()
        .await?;

    // println!("{:?}", response);

    // let resource_record_sets: Vec<ResourceRecordSet> = Vec::new();

    //let x = response.resource_record_sets.iter();

    let resource_record_sets = response.resource_record_sets;

    for record_set in resource_record_sets.iter() {
        if let Some(record_type) = record_set.r#type().into() {
            if record_type == &aws_sdk_route53::types::RrType::A {
                println!("{:?}", record_set);
                println!();
            }
        } else {
            println!("Error parsing record type.");
            // Handle the case where the record type is not present
        }
    }
    Ok(())
}
