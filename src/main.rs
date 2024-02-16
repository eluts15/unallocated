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
    let mut instance_id = String::new();
    let mut public_ip_address = String::new();
    let mut domain_name = String::new();
    let mut a_record = Vec::new();
    let mut r_type = String::new();

    match search_hosted_zones(&r53_client).await {
        Ok(zone_ids) => {
            let ips = list_all_ec2_ips(&ec2_client).await;

            match ips {
                Ok(ips) => {
                    for (id, ip) in ips {
                        instance_id = id;
                        public_ip_address = ip;
                        println!(
                            "Instance ID: {},\nIP Address: {}\n",
                            instance_id, public_ip_address
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching EC2 IPs: {}", err);
                }
            }

            // Janky way of passing the zone_id to the function.
            let a_records = list_all_resource_record_sets(&r53_client, &zone_ids).await;

            match a_records {
                Ok(a_records) => {
                    for (name, record, record_type) in a_records {
                        domain_name = name;
                        a_record = record;
                        r_type = record_type;
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching records fom Route53: {}", err);
                }
            }

            let id = instance_id;
            let public_ip_address = public_ip_address;
            let a_record = a_record;
            let r_type = r_type;
            let domain_name = domain_name;

            // Determine if the A record in Route53 is attached to an existing instance.
            for record in &a_record {
                if record == &public_ip_address {
                    println!("No unallocated IP addresses found in Route53.");
                } else {
                    println!("The record {:?} of type: {:?} for {:?} appears to be unallocated. Consider deleting the record.", record, r_type, domain_name);
                }
            }
        }
        Err(err) => eprintln!("Error fetching resources {:?}", err),
    }

    println!("Done.");
}
