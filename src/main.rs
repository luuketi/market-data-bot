mod timeseries;

use std::error::Error;
use crate::timeseries::client::TimeSeriesClient;
use chrono::{ NaiveDate };

fn main() -> Result<(), Box<dyn Error>> {
    const API_KEY: &str = "lsxqS-rzK0F50T4VqrqG";

    let start_date = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str("2024-06-01", "%Y-%m-%d")?;

    let ts_client = TimeSeriesClient::new(API_KEY.to_string());
    let response = ts_client.get("BTCUSD".to_string(),
                                 format!("{}", start_date.format("%Y-%m-%d")),
                                 format!("{}", end_date.format("%Y-%m-%d")))?;

    println!("{response:?}");

    Ok(())
}
