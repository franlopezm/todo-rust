use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub service_host: String,
    pub service_port: u16,
}

pub fn get() -> EnvConfig {
    envy::from_env::<EnvConfig>().expect("SERVICE_HOST and SERVICE_PORT are env vars required.")
}
