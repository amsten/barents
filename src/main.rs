extern crate dotenv;

use barents::database::{configuration, postgres};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, warn};
use std::{env, error::Error};
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    debug!("Reading configuration file for Postgres");
    let config = configuration::get_configuration()?;

    let db = configuration::DatabaseSettings {
        username: config.database.username.to_string(),
        password: config.database.password.to_string(),
        host: config.database.host.to_string(),
        port: config.database.port,
        database_name: config.database.database_name.to_string(),
    };
    let connection_string = db.connection_string();
    debug!("Connection string: {}", connection_string);
    let db = postgres::BarentsPostgresConnection::new(connection_string);

    let mut ais = barents::live_ais::ais_stream::AisLiveAPI::new(
        "client_credentials".to_owned(),
        env::var("CLIENT_ID").unwrap().to_owned(),
        env::var("CLIENT_SECRET").unwrap().to_owned(),
        barents::live_ais::ais_stream::ScopeType::Ais,
    );

    let last_hour = ais
        .get_latest_ais(Utc::now() - chrono::Duration::hours(1))
        .await?;
    let status_code: i32;
    let content_length: i64;

    match i32::try_from(last_hour.status_code) {
        Ok(val) => status_code = val,
        Err(_) => {
            warn!("Failed to convert the status code into i16 data type. Defaulting to 0");
            status_code = 0;
        }
    };
    match i64::try_from(last_hour.content_length.unwrap_or_default()) {
        Ok(val) => content_length = val,
        Err(_) => {
            warn!("Failed to convert the content length into i64 data type. Defaulting to 0");
            content_length = 0;
        }
    };

    db.insert_request_log(last_hour.api_endpoint, status_code, content_length)
        .await?;

    Ok(())
}
