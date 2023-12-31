use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::profile::profile_file::{ProfileFileKind, ProfileFiles};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use config::Config;
use std::ffi::c_long;

mod settings;

#[tokio::main]
async fn main() {
    env_logger::init();
    let settings = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build();

    let bucket_name = settings.as_ref().unwrap().get::<String>("aws.bucket_name");
    let object_key = settings.as_ref().unwrap().get::<String>("aws.object_key");

    let credentials_provider = DefaultCredentialsChain::builder()
        .profile_name("personal")
        .build()
        .await;

    let region_provider = DefaultRegionChain::builder()
        .profile_name("personal")
        .build();
    let aws_config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&aws_config);
    let head_object = client
        .head_object()
        .bucket(bucket_name.unwrap())
        .key(object_key.unwrap())
        .send()
        .await;
    println!("head object: {:?}", head_object);
}
