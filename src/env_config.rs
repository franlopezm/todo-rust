use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub service_ip: String,
    pub service_port: u16,
}

pub fn get() -> EnvConfig {
    envy::from_env::<EnvConfig>().expect("SERVICE_IP and SERVICE_PORT are env vars required.")
}
