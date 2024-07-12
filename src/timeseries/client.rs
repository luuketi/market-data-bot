use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::quote::quote::Quote;

const TIMESERIES_ENDPOINT: &str = "https://marketdata.tradermade.com/api/v1/timeseries";

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeries {
    base_currency: String,
    quote_currency: String,
    start_date: String,
    end_date: String,
    pub(crate) quotes: Vec<Quote>

}

#[derive(Clone)]
pub struct TimeSeriesClient {
    api_key: String,
    http_client: reqwest::Client,
}

impl TimeSeriesClient {

    pub fn new(api_key: String) -> Self {
        TimeSeriesClient{api_key: api_key, http_client: reqwest::Client::new()}
    }
    pub async fn get(&self, currencies: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> anyhow::Result<TimeSeries> {
        let start = format!("{}", start_date.format("%Y-%m-%d"));
        let end = format!("{}", end_date.format("%Y-%m-%d"));

        let url = format!("{TIMESERIES_ENDPOINT}?currency={currencies}&start_date={start}&end_date={end}&api_key={}", self.api_key);
        Ok(self.http_client.get(&url).send().await?.json::<TimeSeries>().await?)
    }
}