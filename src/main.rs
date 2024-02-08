mod config;
mod tasks;

use crate::tasks::list_all_ec2_ips;
use crate::tasks::list_all_resource_record_sets;

#[tokio::main]
async fn main() {
    let ips = list_all_ec2_ips().await.unwrap();

    let hosted_zone_id: &str = "Z0876296ZKE63PZOBDCA"; // Adjust as needed
                                                       //let hosted_zone_name: &str = "dev-nexgen-ethanl-sandbox.4thdown.co";

    let record_sets = list_all_resource_record_sets(hosted_zone_id).await.unwrap(); // panic!
    let a_records: Vec<String> = record_sets
        .into_iter()
        .filter(|rs| rs.r#type == "A".into() || rs.r#type == "AAAA".into())
        .flat_map(|rs| {
            rs.resource_records
                .unwrap_or_default()
                .into_iter()
                .map(|rr| rr.value)
        })
        .collect();

    println!("records of type A or AAAA: {:?}", a_records);

    let unassociated_ips: Vec<String> = a_records
        .into_iter()
        .filter(|ip| !ips.contains(ip))
        .collect();

    println!("unassociated_ips: {:?}", unassociated_ips);

    if !unassociated_ips.is_empty() {
        println!(
            "Zone: {}, Unassociated IPs: {:?}",
            hosted_zone_id, unassociated_ips
        );
    }

    println!("Main Finished Executing..");
}
