use aws_sdk_ec2::{Client, Error};

pub async fn list_all_ec2_ips(client: &Client) -> Result<Vec<(String, String)>, Error> {
    let response = client.describe_instances().to_owned().send().await?;

    let reservations = response.reservations;

    let mut instance_info: Vec<(String, String)> = Vec::new();

    if let Some(reservations) = reservations {
        for reservation in reservations {
            if let Some(instances) = reservation.instances {
                for instance in instances {
                    if let Some(instance_id) = instance.instance_id {
                        if let Some(public_ip) = instance.public_ip_address {
                            instance_info.push((instance_id, public_ip));
                        }
                    }
                }
            }
        }
    } else {
        println!("Error: Error fetching instance info.");
    }
    //let return_instance_info = instance_info.clone();
    Ok(instance_info)
}
