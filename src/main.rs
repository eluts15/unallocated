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

    println!("Main Finished Executing..");
}
