use config::Config;
use log::debug;

mod settings;

fn main() {
    env_logger::init();
    let settings = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build();
    debug!("aws region: {:?}", settings.unwrap().get::<String>("aws.region"));
}
