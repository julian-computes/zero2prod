use zero2prod::configuration::get_configuration;
use zero2prod::prelude::*;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    Ok(run(configuration.application_port).await?)
}
