mod fetch_records;
mod linode_api_read_only;
mod list_ec2_ips;

use crate::fetch_records::{list_all_resource_record_sets, search_hosted_zones};
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

//use log::{debug, error, info, log_enabled, Level};

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

    let token = env::var("LINODE_API_TOKEN").expect("LINODE_API_TOKEN in .env file.");

    match search_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            // Fetch all "A" records.
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

                    // At this point we should have the data we need from both Linode
                    let linode_instance_info = linode_instance_info;

                    println!("Fetching public IPs for associated EC2 Instances.");
                    let ec2_instance_info =
                        list_all_ec2_ips(&ec2_client).await.unwrap_or_else(|err| {
                            eprintln!("Error fetching EC2 instance info: {}", err);
                            Vec::new() // Return an empty vector as a fallback value
                        });

                    let mut ocurrences_ok = 0;
                    let mut ocurrences_unexpected = 0;
                    let mut ocurrences_info = 0;
                    println!();
                    // Print column headers
                    println!(
                        "{: <20} | {: <50} | {: <10}",
                        "Address", "Domain Name", "Status"
                    );
                    println!();

                    // Compare the allocated addresses and see if a record exists.
                    for (name, record, _) in &a_records {
                        // Extract IP addresses from the record, if available
                        let addresses: Vec<String> =
                            record.iter().map(|ip| ip.to_string()).collect();
                        let address = if addresses.is_empty() {
                            "No IP".to_string()
                        } else {
                            addresses.join("\n")
                        };
                        // Determine status based on existing logic
                        let status = if !addresses.is_empty()
                            && linode_instance_info.iter().any(|(_, ip)| ip == &address)
                            || ec2_instance_info.iter().any(|(_, ip)| ip == &address)
                        {
                            "OK"
                        } else if !addresses.is_empty()
                            && !linode_instance_info.iter().any(|(_, ip)| ip == &address)
                            && !ec2_instance_info.iter().any(|(_, ip)| ip == &address)
                        {
                            "UNEXPECTED"
                        } else {
                            "INFO"
                        };

                        match status {
                            "OK" => ocurrences_ok += 1,
                            "UNEXPECTED" => ocurrences_unexpected += 1,
                            "INFO" => ocurrences_info += 1,
                            _ => {}
                        }

                        // Print record details in columns
                        println!("{: <20} | {: <50} | {}", address, name, status);
                    }
                    println!();
                    // Printing the occurrences
                    println!("{} OK", ocurrences_ok);
                    println!("{} May require attention.", ocurrences_unexpected);
                    println!("{} Not associated with an 'A' record.", ocurrences_info);

                    let total_number_of_records = a_records.len();
                    println!("Iterated over {} records.", total_number_of_records);
                    println!("Done");
                }
                Err(err) => {
                    eprintln!("Error fetching records fom Route53: {}", err);
                }
            }

            // Determine if the A record in Route53 is attached to an existing instance.
        }
        Err(err) => eprintln!("Error fetching resources {:?}, Check Permission Set", err),
    }
    Ok(())
}
