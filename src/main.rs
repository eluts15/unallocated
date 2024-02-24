mod fetch_records;
mod linode_api_read_only;
mod list_ec2_ips;

use crate::fetch_records::{fetch_all_resource_record_sets, search_hosted_zones};
use crate::linode_api_read_only::{extract, list_linode_instances};
use crate::list_ec2_ips::list_all_ec2_ips;

use aws_config::BehaviorVersion;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_sdk_ec2::config::{Credentials, Region};
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_route53::Client as Route53Client;
use dotenv::dotenv;
use serde_json::to_string_pretty;
use std::env;

// Allow to pass non-default aws credentials.
#[derive(Debug)]
struct StaticCredentials {
    access_key_id: String,
    secret_access_key: String,
}

impl StaticCredentials {
    pub fn new() -> Self {
        dotenv().ok();
        let access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("access_key_id");
        let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("secret access key");

        Self {
            access_key_id: access_key_id.trim().to_string(),
            secret_access_key: secret_access_key.trim().to_string(),
        }
    }

    async fn load_credentials(&self) -> aws_credential_types::provider::Result {
        Ok(Credentials::new(
            self.access_key_id.clone(),
            self.secret_access_key.clone(),
            None,
            None,
            "StaticCredentials",
        ))
    }
}

impl ProvideCredentials for StaticCredentials {
    fn provide_credentials<'a>(
        &'a self,
    ) -> aws_credential_types::provider::future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        aws_credential_types::provider::future::ProvideCredentials::new(self.load_credentials())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // AWS credential setup.
    let shared_config = aws_config::SdkConfig::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(
            env::var("AWS_REGION").expect("AWS_REGION in .env file."),
        ))
        .credentials_provider(SharedCredentialsProvider::new(StaticCredentials::new()))
        .build();

    let r53_client = Route53Client::new(&shared_config);
    let ec2_client = Ec2Client::new(&shared_config);

    // Linode
    let token = env::var("LINODE_API_TOKEN").expect("LINODE_API_TOKEN in .env file.");

    match search_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            let resource_record_sets = fetch_all_resource_record_sets(&r53_client, &zone_ids).await;

            match resource_record_sets {
                Ok(ref records) => {
                    println!("Fetching public IPs for associated Linode Instances.");
                    let json_response = match list_linode_instances(token).await {
                        Ok(response) => response,
                        Err(err) => {
                            eprintln!("Error with linode GET request: {}", err);
                            return Ok(());
                        }
                    };
                    let formatted_json_response = to_string_pretty(&json_response)?;

                    // Linode
                    let mut linode_instance_info: Vec<(String, String)> = Vec::new();
                    match extract(&formatted_json_response).await {
                        Ok(data) => {
                            for (id, _, ipv4) in &data {
                                linode_instance_info.push((id.to_string(), ipv4.to_string()));
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading linode instance info: {}", err);
                        }
                    };

                    let linode_instance_info = linode_instance_info;

                    println!("Fetching public IPs for associated EC2 Instances.");
                    println!();
                    let ec2_instance_info =
                        list_all_ec2_ips(&ec2_client).await.unwrap_or_else(|err| {
                            eprintln!("Error fetching EC2 instance info: {}", err);
                            Vec::new() // Return an empty vector as a fallback value
                        });

                    let mut occurrences_ok = 0;
                    let mut occurrences_unexpected = 0;
                    let mut occurrences_info = 0;

                    // Print column headers
                    println!(
                        "{: <20} | {: <50} | {: <10}",
                        "Address", "Domain Name", "Status"
                    );

                    // Compare the allocated addresses and see if a record exists
                    for record in records {
                        // Ensure the record is an A record
                        if let aws_sdk_route53::types::RrType::A = record.r#type {
                            // Extract IP addresses from the record
                            let ip = record.resource_records.as_ref().map_or(
                                "No IP".to_string(),
                                |records| {
                                    records
                                        .first()
                                        .map_or("No IP".to_string(), |record| record.value.clone())
                                },
                            );

                            let domain_name = record.name.clone();
                            let mut is_ok = false;

                            if linode_instance_info
                                .iter()
                                .any(|(_, linode_ip)| linode_ip == &ip)
                            {
                                is_ok = true;
                            }

                            if ec2_instance_info.iter().any(|(_, ec2_ip)| ec2_ip == &ip) {
                                is_ok = true;
                            }

                            let status = if is_ok {
                                "OK" // IP is associated with either Linode or EC2 instance
                            } else {
                                "UNEXPECTED" // IP is not associated with either Linode or EC2 instance
                            };
                            // Print record details in columns
                            println!("{: <20} | {: <50} | {}", ip, domain_name, status);

                            // Increment the counters based on the status
                            match status {
                                "OK" => occurrences_ok += 1,
                                "UNEXPECTED" => occurrences_unexpected += 1,
                                _ => (),
                            };
                        }
                    }

                    println!();
                    println!("{} OK.", occurrences_ok);
                    println!("{} May require attention.", occurrences_unexpected);

                    let total_number_of_records = records.len();
                    println!("Iterated over {} records.", total_number_of_records);

                    println!("Done");
                }
                Err(err) => {
                    eprintln!("Error fetching records fom Route53: {}", err);
                }
            }
        }
        Err(err) => eprintln!("Error fetching resources {:?}, Check Permission Set", err),
    }
    Ok(())
}
