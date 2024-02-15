mod unallocated;
use crate::unallocated::fetch_hosted_zones;
use crate::unallocated::list_all_resource_record_sets;
use aws_sdk_route53::Client;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // AWS credential setup.
    dotenv().ok();

    let env_profile = "PROFILE";
    let profile_name = dotenv::var(env_profile).unwrap();
    let shared_config = aws_config::from_env()
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name(profile_name)
                .build(),
        )
        .load()
        .await;

    let client = Client::new(&shared_config);

    //let ips = list_all_ec2_ips().await.unwrap();
    //

    match fetch_hosted_zones(&client).await {
        Ok(zone_ids) => {
            // Janky way of passing the zone_id to the function.
            let record_sets = list_all_resource_record_sets(&client, &zone_ids).await;
            println!("{:?}", record_sets);
        }
        Err(err) => eprintln!("Error fetching record sets {:?}", err),
    }

    //println!("Zone ID: {}", zone_id);
    //println!("Zone Name: {}", zone_name);

    //let record_sets = list_all_resource_record_sets(&client, hosted_zone_id).await; //panic!

    // let a_records: Vec<String> = record_sets
    //     .into_iter()
    //     .filter(|rs| rs.r#type == "A".into() || rs.r#type == "AAAA".into())
    //     .flat_map(|rs| {
    //         rs.resource_records
    //             .unwrap_or_default()
    //             .into_iter()
    //             .map(|rr| rr.value)
    //     })
    //     .collect();

    // println!("records of type A or AAAA: {:?}", a_records);

    //let unassociated_ips: Vec<String> = a_records
    //    .into_iter()
    //    .filter(|ip| !ips.contains(ip))
    //    .collect();

    //println!("unassociated_ips: {:?}", unassociated_ips);

    //if !unassociated_ips.is_empty() {
    //    println!(
    //        "Zone: {}, Unassociated IPs: {:?}",
    //        hosted_zone_id, unassociated_ips
    //    );
    //}

    println!("Main Finished Executing..");
}
