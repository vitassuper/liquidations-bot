mod config;
mod models;
mod schema;

use bigdecimal::BigDecimal;
use config::Config;
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use tungstenite::{connect, Message};

const WEBSOCKET_URL: &str = "wss://fstream.binance.com/ws/!forceOrder@arr";

#[derive(Serialize, Deserialize, Debug)]
struct LiquidationEvent {
    #[serde(alias = "e")]
    event_name: String,
    #[serde(alias = "E")]
    event_time: u64,
    #[serde(alias = "o")]
    liquidation: Liquidation,
}

#[derive(Serialize, Deserialize, Debug)]
struct Liquidation {
    #[serde(alias = "s")]
    symbol: String,
    #[serde(alias = "S")]
    side: String,
    #[serde(alias = "q")]
    quantity: BigDecimal,
}

fn establish_connection(url: &String) -> PgConnection {
    PgConnection::establish(url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

fn create_new_liquidation(
    pg_connection: &mut PgConnection,
    liquidation_event: LiquidationEvent,
) -> QueryResult<usize> {
    diesel::insert_into(schema::liquidations::table)
        .values(models::NewLiquidation {
            symbol: liquidation_event.liquidation.symbol,
            side: liquidation_event.liquidation.side,
            quantity: liquidation_event.liquidation.quantity,
        })
        .execute(pg_connection)
}

fn main() {
    let config = Config::new().unwrap();
    let mut pg_connection = establish_connection(&config.database_url);

    let (mut ws_stream, _) = connect(WEBSOCKET_URL).expect("Error connect");

    loop {
        let message = ws_stream.read().expect("Error read");

        if let Message::Text(text) = &message {
            match serde_json::from_str::<LiquidationEvent>(&text.to_string()) {
                Ok(liquidation_event) => {
                    if let Err(error) =
                        create_new_liquidation(&mut pg_connection, liquidation_event)
                    {
                        println!("Error writing to DB: {}", error)
                    }
                }
                Err(error) => {
                    println!("Error deserializing JSON: {}, {}", error, &text.to_string())
                }
            }
        }
    }
}
