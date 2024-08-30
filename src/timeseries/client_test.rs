
#[test]
fn get_currencies_pairs_test() {
    let pairs = ["GBPUSD".to_string(),"GBPUSDT".to_string(),"GBPUST".to_string()];

    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(client::CURRENCIES_PATH);
        then.status(200).json_body(json!({"available_currencies":pairs}));
    });
    let url = format!("http://{}", server.address().to_string());

    let ts_client = client::TimeSeriesClient::new("some-api-key".to_string(), url);

    let expected_currencies_pairs = client::AvailableCurrenciesPairs{available_currencies: Vec::from(pairs)};

    let currencies_pairs = tokio_test::block_on(ts_client.get_currencies_pairs()).unwrap();

    assert!(expected_currencies_pairs.available_currencies.iter()
       .zip(currencies_pairs.available_currencies)
       .all(|(a,b)| *a == *b));
}

#[test]
fn get_timeseries_test() {
    let start_date = Utc::now();
    let end_date = Utc::now();

    let expected_ts = client::TimeSeries{
        base_currency: "USD".to_string(),
        quote_currency: "JPY".to_string(),
        start_date: format!("{}", start_date.format("%Y-%m-%d")),
        end_date:  format!("{}", end_date.format("%Y-%m-%d")),
        quotes: Vec::from([
            Quote{
                open: Some(1.),
                close: Some(1.),
                high: Some(1.),
                low: Some(1.),
                date: format!("{}", start_date.format("%Y-%m-%d")),
            }
        ])
    };

    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(client::TIMESERIES_PATH);
        then.status(200).json_body(json!(expected_ts));
    });
    let url = format!("http://{}", server.address().to_string());

    let ts_client = client::TimeSeriesClient::new("some-api-key".to_string(), url);
    let ts = tokio_test::block_on(ts_client.get_timeseries("USDJPY", start_date, end_date)).unwrap();


    assert_eq!(expected_ts.base_currency, ts.base_currency);
    assert_eq!(expected_ts.quote_currency, ts.quote_currency);
    assert_eq!(expected_ts.start_date, ts.start_date);
    assert_eq!(expected_ts.end_date, ts.end_date);
    assert!(expected_ts.quotes.iter().zip(ts.quotes).all(assert_quotes));
}

#[test]
fn get_timeseries_with_empty_quotes_test() {
    let start_date = Utc::now();
    let end_date = Utc::now();

    let mocked_ts = client::TimeSeries{
        base_currency: "USD".to_string(),
        quote_currency: "JPY".to_string(),
        start_date: format!("{}", start_date.format("%Y-%m-%d")),
        end_date:  format!("{}", end_date.format("%Y-%m-%d")),
        quotes: Vec::from([
            Quote{
                open: Some(1.),
                close: Some(1.),
                high: Some(1.),
                low: Some(1.),
                date: format!("{}", start_date.format("%Y-%m-%d")),
            },
            Quote{
                open: None,
                close: None,
                high: None,
                low: None,
                date: format!("{}", start_date.format("%Y-%m-%d")),
            }
        ])
    };

    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path(client::TIMESERIES_PATH);
        then.status(200).json_body(json!(mocked_ts));
    });
    let url = format!("http://{}", server.address().to_string());

    let ts_client = client::TimeSeriesClient::new("some-api-key".to_string(), url);
    let ts = tokio_test::block_on(ts_client.get_timeseries("USDJPY", start_date, end_date)).unwrap();

    let expected_ts = client::TimeSeries{
        base_currency: "USD".to_string(),
        quote_currency: "JPY".to_string(),
        start_date: format!("{}", start_date.format("%Y-%m-%d")),
        end_date:  format!("{}", end_date.format("%Y-%m-%d")),
        quotes: Vec::from([
            Quote{
                open: Some(1.),
                close: Some(1.),
                high: Some(1.),
                low: Some(1.),
                date: format!("{}", start_date.format("%Y-%m-%d")),
            },
        ])
    };
    assert_eq!(expected_ts.base_currency, ts.base_currency);
    assert_eq!(expected_ts.quote_currency, ts.quote_currency);
    assert_eq!(expected_ts.start_date, ts.start_date);
    assert_eq!(expected_ts.end_date, ts.end_date);
    assert!(expected_ts.quotes.iter().zip(ts.quotes).all(assert_quotes));
}
