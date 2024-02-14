//use aws_sdk_route53::types::ResourceRecordSet;
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
    _client: &Client,
    hosted_zone_id: &str,
) -> Result<(), Error> {
    //
    //
    let hosted_zone_id = &hosted_zone_id;

    if hosted_zone_id.is_empty() {
        println!("Zone Error: {:?}", hosted_zone_id);
    } else {
        println!("Zone ID found: {:?}", hosted_zone_id);
    }

    //let mut record_sets = Vec::new();
    //let mut start_record_name = None;

    //println!("Attempting to fetch records...");
    //loop {
    //    println!("Inside loop for fetching records");
    //    let resp = client
    //        .list_resource_record_sets()
    //        .hosted_zone_id(*hosted_zone_id)
    //        .start_record_name(start_record_name.clone().unwrap_or_default())
    //        .send()
    //        .await?;

    //    println!("Got response back: {:?}", resp); // We don't get here for some reason...

    //    let resource_record_sets = resp.resource_record_sets;
    //    record_sets.extend(resource_record_sets);
    //    if let Some(next_record_name) = resp.next_record_name {
    //        println!("Here");
    //        start_record_name = Some(next_record_name);
    //    } else {
    //        break;
    //    }
    //}

    //println!("Records ->: {:?}", record_sets);

    Ok(())
}
