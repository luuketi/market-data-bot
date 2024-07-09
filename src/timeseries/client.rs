use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesResponse {
    base_currency: String,
    quote_currency: String,
    start_date: String,
    end_date: String,
    quotes: Vec<Quote>

}
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    close: Option<f32>,
    date: String,
}

pub struct TimeSeriesClient {
    api_key: String,
    http_client: reqwest::blocking::Client,
}

impl TimeSeriesClient {

    pub fn new(api_key: String) -> Self {
        TimeSeriesClient{api_key, http_client: reqwest::blocking::Client::builder().build().unwrap()}
    }
    pub fn get(&self, currency: String, start_date: String, end_date: String) -> anyhow::Result<TimeSeriesResponse> {
        const TIMESERIES_ENDPOINT: &str = "https://marketdata.tradermade.com/api/v1/timeseries";
        let url = format!("{TIMESERIES_ENDPOINT}?currency={currency}&start_date={start_date}&end_date={end_date}&api_key={}", self.api_key);
        Ok(self.http_client.get(&url).send()?.json::<TimeSeriesResponse>()?)
    }
}