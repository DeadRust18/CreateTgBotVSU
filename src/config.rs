use dotenvy::dotenv;
use std::env;

pub struct AppConfig {
    pub bot_token: String,
    pub database_url: String,
    //pub http_bind_address: String,
}

impl AppConfig {
    // Загружаем конфигурацию приложения из переменных окружения
    pub fn load_from_environment()-> Self{
        dotenv().ok();
        let bot_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN is not set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        //let http_bind_address = env::var("HTTP_BIND_ADDRESS").expect("HTTP_BIND_ADDRESS is not set");

        Self {
            bot_token,
            database_url,
            //http_bind_address,
        }
    }
}