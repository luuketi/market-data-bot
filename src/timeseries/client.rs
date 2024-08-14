use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::quote::quote::Quote;

pub const ENDPOINT : &str = "https://marketdata.tradermade.com";
pub const TIMESERIES_PATH: &str = "/api/v1/timeseries";
pub const CURRENCIES_PATH: &str = "/api/v1/historical_currencies_list";

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeries {
    pub base_currency: String,
    pub quote_currency: String,
    pub start_date: String,
    pub end_date: String,
    pub quotes: Vec<Quote>
}

impl TimeSeries {
    pub fn is_empty(&self) -> bool {
        self.quotes.is_empty() ||
        self.quotes.iter().any(|quote| quote.high.unwrap() - quote.low.unwrap() == 0f32)
    }

    pub fn delete_empty_quotes(&mut self) {
        self.quotes.retain(|q| !q.is_empty());
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableCurrenciesPairs {
    pub available_currencies: Vec<String>
}

#[derive(Clone)]
pub struct TimeSeriesClient {
    api_key: String,
    http_client: reqwest::Client,
    endpoint: String,
}

impl TimeSeriesClient {

    pub fn new(api_key: String, endpoint: String) -> Self {
        TimeSeriesClient{api_key: api_key, http_client: reqwest::Client::new(), endpoint: endpoint}
    }
    pub async fn get_timeseries(&self, currencies: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> anyhow::Result<TimeSeries> {
        let start = format!("{}", start_date.format("%Y-%m-%d"));
        let end = format!("{}", end_date.format("%Y-%m-%d"));

        let url = format!("{}{TIMESERIES_PATH}?currency={currencies}&start_date={start}&end_date={end}&api_key={}", self.endpoint, self.api_key);
        let mut timeseries = self.http_client.get(&url).send().await?.json::<TimeSeries>().await?;
        timeseries.delete_empty_quotes();
        Ok(timeseries)
    }

    pub async fn get_currencies_pairs(&self) -> anyhow::Result<AvailableCurrenciesPairs> {
        let url = format!("{}{CURRENCIES_PATH}?api_key={}", self.endpoint, self.api_key);
        Ok(self.http_client.get(&url).send().await?.json::<AvailableCurrenciesPairs>().await?)
    }
}
