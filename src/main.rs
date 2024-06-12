use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2Prod::configuration::get_configuration;
use zero2Prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read Configutation");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to PostGres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
