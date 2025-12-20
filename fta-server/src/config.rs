use anyhow::Result;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from_email: String,
    pub smtp_from_name: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub base_url: String,
    pub app_name: String,
}

pub fn load_environment_config() -> Result<Config> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let smtp_host = std::env::var("SMTP_HOST")?;
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER")?;
    let smtp_pass = std::env::var("SMTP_PASS")?;
    let smtp_from_email = std::env::var("SMTP_FROM_EMAIL")?;
    let smtp_from_name =
        std::env::var("SMTP_FROM_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")?;
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;
    let google_redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")?;
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "Financial Tracker".to_string());

    Ok(Config {
        database_url,
        jwt_secret,
        smtp_host,
        smtp_port,
        smtp_user,
        smtp_pass,
        smtp_from_email,
        smtp_from_name,
        google_client_id,
        google_client_secret,
        google_redirect_uri,
        base_url,
        app_name,
    })
}
