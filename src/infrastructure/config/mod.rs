use std::env;

const DB_URL: &'static str = "GROSH_DB_URL"; 
const DB_SCHEMA: &'static str = "GROSH_SPENDINGS_SCHEMA"; 
const BIND_ADDRESS: &'static str = "GROSH_SPENDINGS_BIND_ADDRESS"; 

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub db_schema: String,
    pub bind_address: String,
}

pub fn get_config() -> anyhow::Result<Config> {
    let db_url = env::var(DB_URL)?;
    let db_schema = env::var(DB_SCHEMA)?;
    let bind_address = env::var(BIND_ADDRESS)?;

    let cfg = Config{
        db_url,
        db_schema,
        bind_address,
    };

    Ok(cfg)
}
