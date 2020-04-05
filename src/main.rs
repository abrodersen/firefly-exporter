
mod model;
mod config;
mod client;

use tokio::prelude::*;

use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read_config()?;
    let client = client::Client::new(config.firefly.url, config.firefly.token);
    let time = Utc::now() - chrono::Duration::days(7);
    let transactions = client.get_daily_transactions(time.date().naive_local()).await?;
    println!("{:#?}", transactions);

    Ok(())
}
