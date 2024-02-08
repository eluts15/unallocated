use crate::config::{load_config_ec2, load_config_route53};
use aws_sdk_ec2::{Client as Ec2Client, Error as Ec2Error};
use aws_sdk_route53::types::ResourceRecordSet;
use aws_sdk_route53::{Client as Route53Client, Error as Route53Error};

pub async fn list_all_ec2_ips() -> Result<Vec<String>, Ec2Error> {
    let config = match load_config_ec2().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let ec2 = Ec2Client::new(&config);

    let mut ips = Vec::new();
    let mut next_token = None;

    loop {
        let resp = ec2
            .describe_instances()
            .next_token(next_token.clone().unwrap_or_default())
            .send()
            .await?;

        for reservation in resp.reservations.unwrap() {
            for instance in reservation.instances.unwrap() {
                if let Some(public_ip) = instance.public_ip_address {
                    ips.push(public_ip);
                }
                if let Some(private_ip) = instance.private_ip_address {
                    ips.push(private_ip);
                }
            }
        }
        next_token = resp.next_token;
        if next_token.is_none() {
            break;
        }
    }
    println!("Ran list_all_ec2_ips");

    Ok(ips)
}

pub async fn list_all_resource_record_sets(
    hosted_zone_id: &str,
    //hosted_zone_name: &str,
) -> Result<Vec<ResourceRecordSet>, Route53Error> {
    let config = match load_config_route53().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading AWS SDK configuration: {:?}", e);
            return Err(e); // Convert the error type if necessary
        }
    };

    let route53 = Route53Client::new(&config);

    if hosted_zone_id.is_empty() {
        println!("Zone Error: {:?}", hosted_zone_id);
    } else {
        println!("Zone ID found: {:?}", hosted_zone_id);
    }

    let mut record_sets = Vec::new();
    let mut start_record_name = None;
    println!("Records ->: {:?}", record_sets);

    //let hosted_zone_name = route53.list_hosted_zones_by_name();
    //let name = *hosted_zone_name.get_hosted_zone_id().unwrap_or_default();
    //println!("HostedZoneName: {:?}", hosted_zone_name);
    //

    println!("Attempting to fetch records...");
    loop {
        println!("Inside loop for fetching records");
        let resp = route53
            .list_resource_record_sets()
            .hosted_zone_id(hosted_zone_id)
            .start_record_name(start_record_name.clone().unwrap_or_default())
            .send()
            .await?;

        println!("Got response back: {:?}", resp); // We don't get here for some reason...

        let resource_record_sets = resp.resource_record_sets;
        record_sets.extend(resource_record_sets);
        if let Some(next_record_name) = resp.next_record_name {
            println!("Here");
            start_record_name = Some(next_record_name);
        } else {
            break;
        }
    }

    println!("Ran list_all_resource_record_sets");

    Ok(record_sets)
}
