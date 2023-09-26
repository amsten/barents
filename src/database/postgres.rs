use sqlx::{query, Error, PgPool};

pub struct BarentsPostgresConnection {
    connection_string: String,
}

impl BarentsPostgresConnection {
    pub fn new(connection_string: String) -> Self {
        return BarentsPostgresConnection { connection_string };
    }
    pub async fn insert_request_log(
        self,
        api_endpoint: String,
        status_code: i32,
        number_of_messages: i64,
    ) -> Result<(), Error> {
        let pool = PgPool::connect(&self.connection_string).await?;
        let mut tx = pool.begin().await?;
        query!(
            "INSERT INTO \
          log.requests (api_endpoint, status_code, number_of_messages_received) \
          VALUES ($1, $2, $3)",
            api_endpoint,
            status_code,
            number_of_messages
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
