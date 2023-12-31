use std::collections::HashMap;
use config::Config;

mod settings;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build();
    println!("aws region: {:?}", settings.unwrap().get::<String>("aws.region"));
}
