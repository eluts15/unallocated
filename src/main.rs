mod fetch_records;
mod list_ec2_ips;
use crate::fetch_records::list_all_resource_record_sets;
use crate::fetch_records::search_hosted_zones;
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

    // Used  to check if an "A" record in Route53 is associated with a dangling EC2 instance.
    //let instance_id = String::new();
    let public_ip_address = String::new();
    //let mut domain_name = String::new();
    //let mut a_record = Vec::new();
    //let mut r_type = String::new();

    match search_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            let ips = list_all_ec2_ips(&ec2_client).await;

            match ips {
                Ok(ips) => {
                    for (id, ip) in ips {
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

            let a_records = list_all_resource_record_sets(&r53_client, &zone_ids).await;

            match a_records {
                Ok(a_records) => {
                    let ec2_instance_public_ips = list_all_ec2_ips(&ec2_client).await;

                    // Compare existing EC2 instance IPs in this account
                    match ec2_instance_public_ips {
                        Ok(ec2_instance_public_ips) => {
                            for (name, record, _) in a_records {
                                println!(
                                    "Existing Record found: {:?}. Domain Name: {}",
                                    record, name
                                );

                                if let Some(record_ip) = record.first() {
                                    if !ec2_instance_public_ips
                                        .iter()
                                        .any(|(_, ip)| ip == record_ip)
                                    {
                                        println!("The record {:?} for {} appears to be unallocated. Consider deleting the record. --UNEXPECTED", record, name);
                                    }
                                    if ec2_instance_public_ips
                                        .iter()
                                        .any(|(_, ip)| ip == record_ip)
                                    {
                                        println!(
                                            "The record {:?} appears to be valid.  --OK",
                                            record
                                        );
                                    }
                                } else {
                                    // Handle the case where the record does not contain an IP address
                                    println!("INFO: A record does not contain an IP address for {} --INFO", name);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error fetching EC2 instance IPs: {}", err);
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
}
