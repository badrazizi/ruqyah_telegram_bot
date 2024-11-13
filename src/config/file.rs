use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Error,
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configurations {
    pub files_base_dir: String,
    pub telegram_token: String,
}

impl Configurations {
    pub fn get_config() -> &'static Configurations {
        static CONFIG: OnceLock<Configurations> = OnceLock::new();
        CONFIG.get_or_init(|| match Configurations::read("config.json") {
            Ok(config_file) => config_file,
            Err(err) => {
                println!("Error while reading config file: {}", err);
                let mut c = Configurations::default();
                match c.write("config.json") {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error while writing config.json file: {}", err);
                    }
                };
                return c;
            }
        })
    }

    pub fn read(path: &str) -> Result<Self, Error> {
        if path.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Configuration: configuration path is empty",
            ));
        }

        match PathBuf::from_str(&path) {
            Ok(v) => {
                if !v.exists() || !v.is_file() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("file does not exist: {}", path),
                    ));
                }
            }
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to parse path: {}", err),
                ));
            }
        };

        let config_file = match fs::read_to_string(path) {
            Ok(v) => v,
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to read file: {}", err),
                ));
            }
        };

        match serde_json::from_str::<Self>(&config_file) {
            Ok(config) => {
                return Ok(config);
            }
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to parse file: {}", err),
                ));
            }
        };
    }

    pub fn write(&mut self, path: &str) -> Result<(), Error> {
        if path.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "path is empty",
            ));
        }

        match PathBuf::from_str(&path) {
            Ok(v) => {
                if !v.exists() || !v.is_file() {
                    File::create(v)?;
                    return self.write(path);
                }
            }
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to parse path: {}", err),
                ));
            }
        };

        match serde_json::to_string(self) {
            Ok(v) => {
                match fs::write(path, v) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("Unable to write to file: {}", err),
                        ));
                    }
                };
            }
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to parse file: {}", err),
                ));
            }
        };
    }
}
