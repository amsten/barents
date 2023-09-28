extern crate dotenv;

use barents::database::configuration;
use barents::database::postgres::{DbMethods, insert_aton_data, insert_position_data, insert_request_log, insert_static_data};
use barents::live_ais::response_structs::{
    AISAtonData, AISLatestResponses, AISPositionData, AISStaticData,
};
use barents::live_ais::{ais_stream::AisLiveAPI, response_structs::GetAISLatestResponse};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, warn};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use sqlx::{PgPool, Postgres};
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
use std::{env, error::Error};
use tokio::task;
use tokio::task::JoinHandle;

struct LastHourAISMessage {
    status_code: i32,
    number_of_items: i64,
    ais_response: GetAISLatestResponse,
}

struct SplitAISMessages {
    static_data: Vec<AISStaticData>,
    aton_data: Vec<AISAtonData>,
    position_data: Vec<AISPositionData>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    debug!("Reading configuration file for Postgres");
    let config = configuration::get_configuration()?;

    let db_settings = configuration::DatabaseSettings {
        username: config.database.username.to_string(),
        password: config.database.password.to_string(),
        host: config.database.host.to_string(),
        port: config.database.port,
        database_name: config.database.database_name.to_string(),
    };
    let connection_string = db_settings.connection_string();
    debug!("Connection string: {}", connection_string);
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let dbm = DbMethods {};

    let ais = AisLiveAPI::new(
        "client_credentials".to_owned(),
        env::var("CLIENT_ID").unwrap().to_owned(),
        env::var("CLIENT_SECRET").unwrap().to_owned(),
        barents::live_ais::ais_stream::ScopeType::Ais,
    );

    let last_hour = fetch_last_hours_ais(ais).await?;
    let log_id = insert_request_log(
            connection_pool.clone(),
            last_hour.ais_response.api_endpoint,
            last_hour.status_code,
            last_hour.number_of_items,
        )
        .await?;
    if let Some(messages) = last_hour.ais_response.ais_latest_responses {
        // TODO: Handle errors.
        let split_messages = process_ais_items(messages).unwrap();

        let aton_handle = task::spawn(insert_aton_data(
            connection_pool.clone(),
            split_messages.aton_data,
            log_id.clone(),
        ));
        let static_handle = task::spawn(insert_static_data(
            connection_pool.clone(),
            split_messages.static_data,
            log_id.clone(),
        ));
        let position_handle = task::spawn(insert_position_data(
            connection_pool.clone(),
            split_messages.position_data,
            log_id.clone(),
        ));

        let res = tokio::try_join!(aton_handle, static_handle, position_handle);
        match res {
            Ok(..) => {
                debug!("Threads completed");
            }
            Err(error) => warn!("There was an error in one of the threads: {:?}", error)
        }
        // let static_data = split_messages.static_data.clone();
        // insert_static_data(
        //     connection_pool.clone(),
        //     split_messages.static_data,
        //     log_id.clone(),
        // ).await?;
    }

    Ok(())
}

fn process_ais_items(ais_items: AISLatestResponses) -> Result<SplitAISMessages, Box<dyn Error>> {
    let static_data: Arc<Mutex<Vec<AISStaticData>>> = Arc::new(Mutex::new(Vec::new()));
    let aton_data: Arc<Mutex<Vec<AISAtonData>>> = Arc::new(Mutex::new(Vec::new()));
    let position_data: Arc<Mutex<Vec<AISPositionData>>> = Arc::new(Mutex::new(Vec::new()));

    ais_items
        .par_iter()
        .for_each(|item| match &item.type_field {
            None => {}
            Some(item_type) => {
                if item_type.eq_ignore_ascii_case("staticdata") {
                    let mut res = static_data.lock().unwrap();
                    let stat: AISStaticData = (item).into();
                    res.push(stat);
                } else if item_type.eq_ignore_ascii_case("aton") {
                    let mut res = aton_data.lock().unwrap();
                    let aton: AISAtonData = (item).into();
                    res.push(aton);
                } else if item_type.eq_ignore_ascii_case("position") {
                    let mut res = position_data.lock().unwrap();
                    let position: AISPositionData = (item).into();
                    res.push(position);
                }
            }
        });
    let static_data = static_data.lock().unwrap().clone();
    let aton_data = aton_data.lock().unwrap().clone();
    let position_data = position_data.lock().unwrap().clone();

    let split_data = SplitAISMessages {
        static_data,
        aton_data,
        position_data,
    };

    Ok(split_data)
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
