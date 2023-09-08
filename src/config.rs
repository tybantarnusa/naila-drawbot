use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigData {
    pub config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub naila_dir: String,
    pub image_filename: String,
    pub tunnel_filename: String,
    pub delay: u64,
}