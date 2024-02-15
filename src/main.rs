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

    // let ec2_instance_info = list_all_ec2_ips(&ec2_client).await;
    // _ = ec2_instance_info;

    match fetch_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            let ips = list_all_ec2_ips(&ec2_client).await;

            match ips {
                Ok(ips) => {
                    // `ips` is now a Vec<(String, String)>
                    for (instance_id, ip) in ips {
                        // Use `instance_id` and `ip` here
                        println!("Instance ID: {},\nIP Address: {}\n", instance_id, ip);
                    }
                }
                Err(err) => {
                    // Handle the error here
                    eprintln!("Error fetching EC2 IPs: {}", err);
                }
            }
            // Janky way of passing the zone_id to the function.
            let a_records = list_all_resource_record_sets(&r53_client, &zone_ids).await;

            match a_records {
                Ok(a_records) => {
                    for (name, record, record_type) in a_records {
                        println!(
                            "Name: {:?}\nRecords: {:?}\nRecord_Type: {:?}\n",
                            name, record, record_type
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching records fom Route53: {}", err);
                }
            }
        }
        Err(err) => eprintln!("Error fetching record sets {:?}", err),
    }

    println!("Done.");
}
