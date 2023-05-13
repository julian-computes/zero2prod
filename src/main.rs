use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::prelude::*;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to get DB connection");
    Ok(run(configuration.application_port, connection).await?)
}
