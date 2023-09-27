extern crate dotenv;

use barents::database::configuration;
use barents::database::postgres::BarentsPostgresConnection;
use barents::live_ais::{ais_stream::AisLiveAPI, response_structs::GetAISLatestResponse};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, warn};
use std::convert::TryFrom;
use std::{env, error::Error};

struct LastHourAISMessage {
    status_code: i32,
    number_of_items: i64,
    ais_response: GetAISLatestResponse,
}

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
    let db = BarentsPostgresConnection::new(connection_string);

    let ais = AisLiveAPI::new(
        "client_credentials".to_owned(),
        env::var("CLIENT_ID").unwrap().to_owned(),
        env::var("CLIENT_SECRET").unwrap().to_owned(),
        barents::live_ais::ais_stream::ScopeType::Ais,
    );

    let last_hour = fetch_last_hours_ais(ais).await?;
    let log_id = db
        .insert_request_log(
            last_hour.ais_response.api_endpoint,
            last_hour.status_code,
            last_hour.number_of_items,
        )
        .await?;

    // Lets insert all the items into the database.
    if last_hour.ais_response.ais_latest_responses.is_some() {
        db.insert_ais_items(last_hour.ais_response.ais_latest_responses.unwrap(), log_id)
            .await?;
    }

    Ok(())
}

async fn fetch_last_hours_ais(mut ais: AisLiveAPI) -> Result<LastHourAISMessage, Box<dyn Error>> {
    let last_hour = ais
        .get_latest_ais(Utc::now() - chrono::Duration::hours(1))
        .await?;

    let status_code: i32;
    let number_of_items: i64;

    match i32::try_from(last_hour.status_code) {
        Ok(val) => status_code = val,
        Err(_) => {
            warn!("Failed to convert the status code into i16 data type. Defaulting to 0");
            status_code = 0;
        }
    };
    match i64::try_from(last_hour.content_length.unwrap_or_default()) {
        Ok(val) => number_of_items = val,
        Err(_) => {
            warn!("Failed to convert the content length into i64 data type. Defaulting to 0");
            number_of_items = 0;
        }
    };

    Ok(LastHourAISMessage {
        status_code,
        number_of_items,
        ais_response: last_hour,
    })
}
