use std::env;

use dotenvy::dotenv;
use tracing::error;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub http_port: String,
    pub https_port: String,
    pub front_end_url: String,

    pub password_salt: String,

    pub public_path: String,
    pub storage_path: String,
    pub private_storage_path: String,
    pub cert_path: String,
    pub log_path: String,
    pub db_path: String,
    pub migration_path: String,

    pub db_namespace: String,
    pub db_name: String,
    pub session_expiration: i64,
    pub session_secure_key: String,

    pub max_body_limit: usize,
    pub rows_per_page: usize,
}

#[cfg(debug_assertions)]
pub static RUNTIME_STACK_SIZE: usize = 20 * 1024 * 1024; // 20MiB in debug mode
#[cfg(not(debug_assertions))]
pub static RUNTIME_STACK_SIZE: usize = 10 * 1024 * 1024; // 10MiB in release mode

pub static RUNTIME_MAX_BLOCKING_THREADS: usize = 512;

pub const SESSION_USER_KEY: &str = "user";
pub const SESSION_ACCESS_KEY: &str = "access";

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        Self {
            host: get_env("HOST"),
            http_port: get_env("HTTP_PORT"),
            https_port: get_env("HTTPS_PORT"),
            password_salt: get_env("PASSWORD_SALT"),
            db_path: get_env("DB_PATH"),
            db_namespace: get_env("DB_NAMESPACE"),
            db_name: get_env("DB_NAME"),
            session_expiration: get_env("SESSION_EXPIRATION_IN_MINUTES")
                .trim()
                .parse::<i64>()
                .unwrap_or(24 * 60),
            session_secure_key: get_env("SESSION_SECURE_KEY"),
            front_end_url: get_env("FRONT_END_URL"),
            max_body_limit: get_env("MAX_BODY_LIMIT")
                .trim()
                .parse::<usize>()
                .unwrap_or(104_857_600),
            rows_per_page: get_env("ROWS_PER_PAGE")
                .trim()
                .parse::<usize>()
                .unwrap_or(10),
            public_path: get_env("PUBLIC_PATH"),
            storage_path: get_env("STORAGE_PATH"),
            private_storage_path: get_env("PRIVATE_STORAGE_PATH"),
            cert_path: get_env("CERT_PATH"),
            log_path: get_env("LOG_PATH"),
            migration_path: get_env("MIGRATION_PATH"),
        }
    }
}

fn get_env(name: &str) -> String {
    env::var(name)
        .map_err(|_| error!("ENV missing: {name}"))
        .unwrap()
}
