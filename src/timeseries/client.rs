use serde::{Serialize, Deserialize};
use crate::quote::quote::Quote;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeries {
    base_currency: String,
    quote_currency: String,
    start_date: String,
    end_date: String,
    pub(crate) quotes: Vec<Quote>

}


pub struct TimeSeriesClient {
    api_key: String,
    http_client: reqwest::blocking::Client,
}

impl TimeSeriesClient {

    pub fn new(api_key: String) -> Self {
        TimeSeriesClient{api_key, http_client: reqwest::blocking::Client::builder().build().unwrap()}
    }
    pub fn get(&self, currencies: String, start_date: String, end_date: String) -> anyhow::Result<TimeSeries> {
        const TIMESERIES_ENDPOINT: &str = "https://marketdata.tradermade.com/api/v1/timeseries";
        let url = format!("{TIMESERIES_ENDPOINT}?currency={currencies}&start_date={start_date}&end_date={end_date}&api_key={}", self.api_key);
        Ok(self.http_client.get(&url).send()?.json::<TimeSeries>()?)
    }
}