use std::{collections::HashMap, env};

#[derive(Debug)]
pub struct Config {
    pub telegram_bot_token: String,
    pub telegram_chat_id1: String,
    pub telegram_chat_id2: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        dotenvy::dotenv().expect("Can't open .env file");

        let env_var_names = [
            "TELEGRAM_BOT_TOKEN",
            "TELEGRAM_CHAT_ID1",
            "TELEGRAM_CHAT_ID2",
            "DATABASE_URL",
        ];

        let mut config_values: HashMap<String, String> = env_var_names
            .iter()
            .filter_map(|&var| env::var(var).ok().map(|value| (var.to_string(), value)))
            .collect();

        if config_values.len() == env_var_names.len() {
            Ok(Config {
                telegram_bot_token: config_values.remove("TELEGRAM_BOT_TOKEN").unwrap(),
                telegram_chat_id1: config_values.remove("TELEGRAM_CHAT_ID1").unwrap(),
                telegram_chat_id2: config_values.remove("TELEGRAM_CHAT_ID2").unwrap(),
                database_url: config_values.remove("DATABASE_URL").unwrap(),
            })
        } else {
            env_var_names
                .iter()
                .filter(|&&var| !config_values.contains_key(var))
                .for_each(|&var| println!("{var} is not set"));

            Err("Env variables validation was failed")
        }
    }
}
