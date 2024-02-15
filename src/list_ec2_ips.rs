use aws_sdk_ec2::{Client, Error};

pub async fn list_all_ec2_ips(client: &Client) -> Result<(), Error> {
    // let mut ips = Vec::new();
    // let mut next_token = None;

    let response = client.describe_instances().to_owned().send().await?;

    let reservations = response.reservations;

    // let mut instances = Vec::new();

    if let Some(reservations) = reservations {
        for reservation in reservations {
            if let Some(instances) = reservation.instances {
                for instance in instances {
                    if let Some(instance_id) = instance.instance_id {
                        println!("{:?}", instance_id);
                        if let Some(public_ip) = instance.public_ip_address {
                            println!("{:?}", public_ip);
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", response);

    //loop {
    //    let resp = client
    //        .describe_instances()
    //        .next_token(next_token.clone())
    //        .send()
    //        .await?;
    //    for reservation in resp.reservations.unwrap() {
    //        for instance in reservation.instances.unwrap() {
    //            if let Some(public_ip) = instance.public_ip_address {
    //                ips.push(public_ip);
    //            }
    //            if let Some(private_ip) = instance.private_ip_address {
    //                ips.push(private_ip);
    //            }
    //        }
    //    }
    //    next_token = resp.next_token;
    //    if next_token.is_none() {
    //        break;
    //    }
    //}

    Ok(())
}
