use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_sdk_s3::Client;
use config::Config;

mod settings;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build();

    let profile_name = config.as_ref().unwrap().get::<String>("aws.profile_name");
    let client: Client = get_s3_client(profile_name.unwrap()).await;

    let object_size: i64 = get_s3_object_size(client, &config.unwrap()).await;
    println!("object size: {}", object_size);
}

async fn get_s3_object_size(client: Client, configs: &Config) -> i64 {
    let bucket_name = configs.get::<String>("aws.bucket_name").unwrap();
    let object_key = configs.get::<String>("aws.object_key").unwrap();

    let head_object = client
        .head_object()
        .bucket(bucket_name)
        .key(object_key.as_str())
        .send()
        .await;
    let object_size: i64 = head_object.as_ref().unwrap().content_length.unwrap();
    println!("size of {} object is: {}", object_key, object_size);
    println!("head object: {:?}", head_object);
    object_size
}

async fn get_s3_client(profile_name: String) -> Client {
    let credentials_provider = DefaultCredentialsChain::builder()
        .profile_name(profile_name.as_str())
        .build()
        .await;

    let region_provider = DefaultRegionChain::builder()
        .profile_name(profile_name.as_str())
        .build();

    let aws_config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await;

    Client::new(&aws_config)
}
