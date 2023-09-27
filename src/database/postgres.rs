use crate::live_ais::response_structs::AISLatestResponses;
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

    pub async fn insert_ais_items(
        &self,
        ais_items: AISLatestResponses,
        log_id: Uuid,
    ) -> Result<(), Error> {
        let pool = PgPool::connect(&self.connection_string).await?;
        let mut tx = pool.begin().await?;
        for item in ais_items {
            let msg_time_as_date_string = convert_to_datetime_option(item.msgtime);

            query!(
                r#"INSERT INTO ais.ais_latest_response_items (
                    type_field,
                    message_type,
                    mmsi,
                    msgtime,
                    imo_number,
                    call_sign,
                    destination,
                    eta,
                    name,
                    draught,
                    ship_length,
                    ship_width,
                    ship_type,
                    dimension_a,
                    dimension_b,
                    dimension_c,
                    dimension_d,
                    position_fixing_device_type,
                    report_class,
                    log_id
                ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20 )
                "#,
                item.type_field,
                item.message_type,
                item.mmsi,
                msg_time_as_date_string,
                item.imo_number,
                item.call_sign,
                item.destination,
                item.eta,
                item.name,
                item.draught,
                item.ship_length,
                item.ship_width,
                item.ship_type,
                item.dimension_a,
                item.dimension_b,
                item.dimension_c,
                item.dimension_d,
                item.position_fixing_device_type,
                item.report_class,
                log_id,
            ).execute(&mut tx).await?;
        }

        tx.commit().await?;
        pool.close().await;
        Ok(())
    }
}

fn convert_to_datetime_option(input: Option<String>) -> Option<DateTime<Utc>> {
    match input {
        Some(date_string) => match DateTime::parse_from_rfc3339(&date_string) {
            Ok(date) => Some(date.with_timezone(&Utc)), // If successful parsing, convert to DateTime<Utc> and wrap in Some
            Err(_) => None,                             // If parsing fails, return None
        },
        None => None, // If input is None, return None
    }
}
