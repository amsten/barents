extern crate dotenv;

use std::{ error::Error, env };
use dotenv::dotenv;
use log::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    debug!("Fetching token.");
    debug!("Client id: {}", env::var("CLIENT_ID").unwrap());

    let mut ais = barents::live_ais::ais_stream::AisLiveAPI::new(
        "client_credentials".to_owned(),
        env::var("CLIENT_ID").unwrap().to_owned(),
        env::var("CLIENT_SECRET").unwrap().to_owned(),
        barents::live_ais::ais_stream::ScopeType::Ais,
    );
    // ais.fetch_token().await?;
    // ais.fetch_token().await?;
    ais.get_latest_ais().await?;
    
    Ok(())
}
