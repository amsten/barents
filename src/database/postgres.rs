use crate::live_ais::response_structs::{AISAtonData, AISPositionData, AISStaticData};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, Error, PgPool};

pub struct DbMethods {}

impl DbMethods {
    // pub fn new(connection_string: String) -> Self {
    //     return BarentsPostgresConnection {
    //         connection_string,
    //         pool: PgPool::connect(&connection_string).await.unwrap(),
    //     };
    // }

    // pub async fn init(connection_string: String) -> Result<Self, sqlx::Error> {
    //     let pool = PgPool::connect(&connection_string).await?;
    //     Ok(BarentsPostgresConnection {
    //         connection_string,
    //         pool,
    //     })
    // }
    pub async fn insert_request_log(
        &self,
        db_pool: PgPool,
        api_endpoint: String,
        status_code: i32,
        number_of_messages: i64,
    ) -> Result<Uuid, Error> {
        // let pool = PgPool::connect(&self.connection_string).await?;
        let mut tx = db_pool.begin().await?;
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

        Ok(id)
    }

    pub async fn insert_static_data(
        &self,
        db_pool: PgPool,
        static_data: Vec<AISStaticData>,
        log_id: &Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // let pool = PgPool::connect(&self.connection_string).await?;
        let tx = db_pool.begin().await?;

        for data in static_data {
            sqlx::query!(
                "INSERT INTO ais.ais_static_data (
                    type_field, message_type, mmsi, msgtime, imo_number, call_sign, destination, eta, name, draught,
                    ship_length, ship_width, ship_type, dimension_a, dimension_b, dimension_c, dimension_d,
                    position_fixing_device_type, report_class, log_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
                data.type_field, data.message_type, data.mmsi, convert_to_datetime_option(data.msgtime), data.imo_number, data.call_sign,
                data.destination, data.eta, data.name, data.draught, data.ship_length, data.ship_width,
                data.ship_type, data.dimension_a, data.dimension_b, data.dimension_c, data.dimension_d,
                data.position_fixing_device_type, data.report_class, log_id
            ).execute(&db_pool).await.unwrap();
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn insert_aton_data(
        &self,
        db_pool: PgPool,
        aton_data: Vec<AISAtonData>,
        log_id: &Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // let pool = PgPool::connect(&self.connection_string).await?;
        let tx = db_pool.begin().await?;

        for data in aton_data {
            sqlx::query!(
                "INSERT INTO ais.ais_aton_data (
                    type_field, message_type, mmsi, msgtime, dimension_a, dimension_b, dimension_c, dimension_d,
                    type_of_aids_to_navigation, latitude, longitude, name, type_of_electronic_fixing_device, log_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
                data.type_field, data.message_type, data.mmsi, convert_to_datetime_option(data.msgtime), data.dimension_a, data.dimension_b,
                data.dimension_c, data.dimension_d, data.type_of_aids_to_navigation, data.latitude,
                data.longitude, data.name, data.type_of_electronic_fixing_device, log_id
            ).execute(&db_pool).await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn insert_position_data(
        &self,
        db_pool: PgPool,
        position_data: Vec<AISPositionData>,
        log_id: &Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // let pool = PgPool::connect(&self.connection_string).await?;
        let tx = db_pool.begin().await?;

        for data in position_data {
            sqlx::query!(
                "INSERT INTO ais.ais_position_data (
                    type_field, message_type, course_over_ground, ais_class, altitude, latitude, longitude,
                    navigational_status, rate_of_turn, speed_over_ground, true_heading, mmsi, msgtime, log_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
                data.type_field, data.message_type, data.course_over_ground, data.ais_class, data.altitude,
                data.latitude, data.longitude, data.navigational_status, data.rate_of_turn, data.speed_over_ground,
                data.true_heading, data.mmsi, convert_to_datetime_option(data.msgtime), log_id
            ).execute(&db_pool).await?;
        }
        tx.commit().await?;

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
