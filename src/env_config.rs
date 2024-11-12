use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    #[serde(default = "default_host")]
    pub service_host: String,
    #[serde(default = "default_port")]
    pub service_port: u16,
}

pub fn get() -> EnvConfig {
    envy::from_env::<EnvConfig>().expect("SERVICE_HOST and SERVICE_PORT are env vars required.")
}

fn default_port() -> u16 {
    8000
}

fn default_host() -> String {
    String::from("127.0.0.1")
}
