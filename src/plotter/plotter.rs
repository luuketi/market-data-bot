use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration, Utc};
use plotters::prelude::*;
use quote::Quote;
use uuid::Uuid;
use crate::quote::quote;

pub fn plot(data: &Vec<Quote>, title: String) -> Result<String, Box<dyn std::error::Error>> {
    let filename = format!("/tmp/stock-{}.png", Uuid::new_v4());
    let filename2 = filename.clone();
    let root = BitMapBackend::new(&filename2, (3072, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (from_date, to_date) = (
        parse_time(&data.first().unwrap().date) - Duration::days(1),
        parse_time(&data.last().unwrap().date) + Duration::days(1),
    );

    let mut min = data.first().unwrap().low;
    let mut max = data.first().unwrap().high;
    for quote in data {
        if quote.low < min { min = quote.low; }
        if quote.high > max { max = quote.high; }
    }

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(70)
        .y_label_area_size(70)
        .caption(title, ("Calibri", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, min.unwrap()-1f32..max.unwrap()+1f32)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;

    chart.draw_series(
        data.iter().map(|x| {
            CandleStick::new(parse_time(&x.date), x.open.unwrap(), x.high.unwrap(), x.low.unwrap(), x.close.unwrap(), GREEN.filled(), RED, 15)
        }),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    return Ok(filename);
}


fn parse_time(t: &str) -> Date<Utc> {
    Utc
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

