use url::Url;

pub struct Config {
    pub http: HttpConfig,
    pub db_url: String,
}

pub struct HttpConfig {
    pub listen_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            http: HttpConfig {
                listen_port: Self::get_env("LISTEN_PORT", Some("8080"))
                    .unwrap()
                    .parse()
                    .expect("LISTEN_PORT should be a number"),
            },
            db_url: Self::get_env("DATABASE_URL", None).expect("DATABASE_URL should be set"),
        }
    }

    fn get_env(name: &str, default: Option<&str>) -> Option<String> {
        std::env::var(name)
            .ok()
            .or_else(|| default.map(str::to_string))
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}
