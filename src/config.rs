use aws_config::{profile::ProfileFileCredentialsProvider, SdkConfig};

use aws_sdk_ec2::Error as Ec2Error;
use aws_sdk_route53::Error as Route53Error;

use dotenv::dotenv;

pub async fn load_config_ec2() -> Result<SdkConfig, Ec2Error> {
    dotenv().ok();

    let env_profile = "PROFILE";

    let profile_name = dotenv::var(env_profile).unwrap();

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile_name)
        .build();

    let config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .load()
        .await;

    Ok(config)
}

pub async fn load_config_route53() -> Result<SdkConfig, Route53Error> {
    dotenv().ok();

    let env_profile = "PROFILE";

    let profile_name = dotenv::var(env_profile).unwrap();

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile_name)
        .build();

    let config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .load()
        .await;

    Ok(config)
}
