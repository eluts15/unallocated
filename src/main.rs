mod unallocated;
use crate::unallocated::fetch_hosted_zones;
use crate::unallocated::list_all_resource_record_sets;
mod list_ec2_ips;
use crate::list_ec2_ips::list_all_ec2_ips;
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_route53::Client as Route53Client;
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

    let r53_client = Route53Client::new(&shared_config);
    let ec2_client = Ec2Client::new(&shared_config);

    //let ips = list_all_ec2_ips().await.unwrap();
    //

    match fetch_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            // Janky way of passing the zone_id to the function.
            _ = list_all_resource_record_sets(&r53_client, &zone_ids).await;
        }
        Err(err) => eprintln!("Error fetching record sets {:?}", err),
    }

    println!("Running fn list_all_ec2_ips..");
    println!();
    _ = list_all_ec2_ips(&ec2_client).await;

    println!("Main Finished Executing..");
}
