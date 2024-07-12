use chrono::{Duration, NaiveDateTime, DateTime, Utc};
use plotters::prelude::*;
use quote::Quote;
use crate::quote::quote;

const CHART_SIZE: (u32, u32) = (3072, 768);

fn parse_time(t: &str) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M").unwrap().and_utc() }

pub fn plot(data: &Vec<Quote>, title: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer_ = vec![0; usize::try_from(CHART_SIZE.0 * CHART_SIZE.1 * 3).unwrap()];
    {
        let root = BitMapBackend::with_buffer(&mut buffer_, CHART_SIZE).into_drawing_area();
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

        chart.configure_mesh().draw()?;

        chart.draw_series(
            data.iter().map(|x| {
                CandleStick::new(parse_time(&x.date), x.open.unwrap(), x.high.unwrap(), x.low.unwrap(), x.close.unwrap(), GREEN.filled(), RED, 15)
            }),
        )?;

        root.present()?;
    }

    let image = image::RgbImage::from_raw(CHART_SIZE.0, CHART_SIZE.1, buffer_).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )?;

    Ok(bytes)
}