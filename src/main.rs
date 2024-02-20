mod fetch_records;
mod linode_api_read_only;
mod list_ec2_ips;
use crate::fetch_records::{list_all_resource_record_sets, search_hosted_zones};
use crate::linode_api_read_only::{extract, list_linode_instances};
use crate::list_ec2_ips::list_all_ec2_ips;
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_route53::Client as Route53Client;
use dotenv::dotenv;
use serde_json::to_string_pretty;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let token = env::var("LINODE_API_TOKEN").expect("LINODE_API_TOKEN in .env file.");

    match search_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            let a_records = list_all_resource_record_sets(&r53_client, &zone_ids).await;

            match a_records {
                Ok(a_records) => {
                    println!("Fetching public IPs for associated Linode Instances.");
                    let json_response = match list_linode_instances(token).await {
                        Ok(response) => response,
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return Ok(());
                        }
                    };
                    let formatted_json_response = to_string_pretty(&json_response)?;

                    // LINODE
                    let mut linode_instance_info: Vec<(String, String)> = Vec::new();
                    match extract(&formatted_json_response).await {
                        Ok(data) => {
                            for (id, _, ipv4) in &data {
                                linode_instance_info.push((id.to_string(), ipv4.to_string()));
                            }
                        }
                        Err(e) => {
                            eprintln!("Error {}", e);
                        }
                    };

                    println!();
                    println!();

                    // At this point we should have the data we need from both Linode
                    let linode_instance_info = linode_instance_info;
                    let ec2_ips = list_all_ec2_ips(&ec2_client).await;

                    // AWS EC2
                    match ec2_ips {
                        Ok(ec2_ips) => {
                            for (id, ip) in ec2_ips {
                                let instance_id = id;
                                let public_ip_address = ip;
                                println!(
                                    "Instance ID: {}, IP Address: {}",
                                    instance_id, public_ip_address
                                );
                            }
                        }
                        Err(err) => {
                            eprintln!("Error fetching EC2 IPs: {}", err);
                        }
                    }
                    println!();
                    let ec2_instance_info = list_all_ec2_ips(&ec2_client).await?;

                    println!("AWS --> {:?}", ec2_instance_info);

                    // Compare the allocated addresses and see if a record exists.
                    for (name, record, _) in a_records {
                        println!("Existing Record found: {:?}. Domain Name: {}", record, name);

                        if let Some(record_ip) = record.first() {
                            if ec2_instance_info.iter().any(|(_, ip)| ip == record_ip) {
                                println!("The record {:?} appears to be valid.  --OK\n", record);
                            }
                            if linode_instance_info.iter().any(|(_, ip)| ip == record_ip) {
                                println!("The record {:?} appears to be valid.  --OK\n", record);
                            }
                            if !ec2_instance_info.iter().any(|(_, ip)| ip == record_ip) {
                                println!("The record {:?} for {} appears to be unallocated. Consider deleting the record. --UNEXPECTED\n", record, name);
                            }
                            if !linode_instance_info.iter().any(|(_, ip)| ip == record_ip) {
                                println!("The record {:?} for {} appears to be unallocated. Consider deleting the record. --UNEXPECTED\n", record, name);
                            }
                        } else {
                            // Handle the case where the record does not contain an IP address
                            println!(
                                "INFO: A record does not contain an IP address for {} --INFO\n",
                                name
                            );
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching records fom Route53: {}", err);
                }
            }

            // Determine if the A record in Route53 is attached to an existing instance.
        }
        Err(err) => eprintln!("Error fetching resources {:?}, Check Permission Set", err),
    }

    println!("Done.");

    Ok(())
}
