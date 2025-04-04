use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
