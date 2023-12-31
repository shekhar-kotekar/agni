use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_sdk_s3::Client;
use config::Config;

mod settings;

#[tokio::main]
async fn main() {
    env_logger::init();
    let settings = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build();

    let bucket_name = settings.as_ref().unwrap().get::<String>("aws.bucket_name");
    let object_key = settings.as_ref().unwrap().get::<String>("aws.object_key");
    let profile_name = settings.as_ref().unwrap().get::<String>("aws.profile_name");

    let credentials_provider = DefaultCredentialsChain::builder()
        .profile_name(profile_name.as_ref().unwrap().as_str())
        .build()
        .await;

    let region_provider = DefaultRegionChain::builder()
        .profile_name(profile_name.unwrap().as_str())
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
        .key(object_key.as_ref().unwrap())
        .send()
        .await;
    let object_size: i64 = head_object.as_ref().unwrap().content_length.unwrap();
    println!("size of {} object is: {}", object_key.unwrap(), object_size);
    println!("head object: {:?}", head_object);
}
