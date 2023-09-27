use crate::live_ais::response_structs::{AISStaticData};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, Error, PgPool};

pub struct BarentsPostgresConnection {
    connection_string: String,
}

impl BarentsPostgresConnection {
    pub fn new(connection_string: String) -> Self {
        return BarentsPostgresConnection { connection_string };
    }
    pub async fn insert_request_log(
        &self,
        api_endpoint: String,
        status_code: i32,
        number_of_messages: i64,
    ) -> Result<Uuid, Error> {
        let pool = PgPool::connect(&self.connection_string).await?;
        let mut tx = pool.begin().await?;
        let id = query!(
            "INSERT INTO \
          log.requests (api_endpoint, status_code, number_of_messages_received) \
          VALUES ($1, $2, $3) RETURNING id;",
            api_endpoint,
            status_code,
            number_of_messages
        )
        .fetch_one(&mut tx)
        .await?
        .id;
        tx.commit().await?;
        pool.close().await;
        Ok(id)
    }

    pub async fn insert_static_data(&self, static_data: AISStaticData) {
        todo!();
    }
}

// fn convert_to_datetime_option(input: Option<String>) -> Option<DateTime<Utc>> {
//     match input {
//         Some(date_string) => match DateTime::parse_from_rfc3339(&date_string) {
//             Ok(date) => Some(date.with_timezone(&Utc)), // If successful parsing, convert to DateTime<Utc> and wrap in Some
//             Err(_) => None,                             // If parsing fails, return None
//         },
//         None => None, // If input is None, return None
//     }
// }
