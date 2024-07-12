mod timeseries;
mod plotter;
mod quote;

use std::env;
use crate::timeseries::client::{TimeSeries, TimeSeriesClient};
use crate::plotter::plotter::plot;

use chrono::{Duration, Utc};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::InputFile;


const TRADERMADE_APIKEY: &str = "TRADERMADE_APIKEY";
const TELOXIDE_TOKEN: &str = "TELOXIDE_TOKEN";

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show a graph for a currency pair with rates in the last 6M.")]
    Rates{currency_pair: String},
}


#[tokio::main]
async fn main() {
    let tradermade_apykey : String;
    match env::var(TRADERMADE_APIKEY) {
        Ok(val) => tradermade_apykey = val,
        Err(_) => panic!("{TRADERMADE_APIKEY} env var not found"),
    }

    let teloxide_token: String;
    match env::var(TELOXIDE_TOKEN) {
        Ok(val) => teloxide_token = val,
        Err(_) => panic!("{TELOXIDE_TOKEN} env var not found"),
    }

    let ts_client = TimeSeriesClient::new(tradermade_apykey);
    let bot = Bot::new(teloxide_token);

    Command::repl(bot, move |bot, msg, cmd| answer(bot, msg, cmd, ts_client.clone())).await;
}

async fn answer<'a>(
    bot: Bot, msg: Message, cmd: Command, ts_client: TimeSeriesClient) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).send().await?
        },
        Command::Rates{currency_pair} => {
            if currency_pair.is_empty() {
                bot.send_message(msg.chat.id, "Missing currency pair").await?;
                return Ok(());
            }

            let currencies = sanitize_currencies(&currency_pair);

            let end_date = Utc::now() - Duration::days(1);
            let start_date = end_date - Duration::days(180);

            let timeseries = ts_client.get(&currencies, start_date, end_date).await.unwrap();

            let file = create_plot_file(&currencies, &timeseries);

            bot.send_message(msg.chat.id, "Here you have:").await?;
            bot.send_document(msg.chat.id, file).send().await.expect("Failed to send request")
        }
    };
    Ok(())
}

fn create_plot_file(currencies: &str, timeseries: &TimeSeries) -> InputFile {
    let title = format!("{} rates", currencies);
    let graph = plot(&timeseries.quotes, &title).unwrap();
    let file = InputFile::memory(graph).file_name(format!("{title}.png"));
    file
}

fn sanitize_currencies(currencies: &str) -> String {
    currencies.chars().filter(|c| !c.is_whitespace() || *c == '-').collect()
}