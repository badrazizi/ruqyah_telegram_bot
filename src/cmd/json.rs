use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{
    fs::{self, File},
    io::Error,
    path::PathBuf,
    str::FromStr as _,
    sync::Arc,
};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use tokio::sync::RwLock;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSON {
    pub welcome: String,
    pub options: Vec<Options>,
    pub keyboards: Vec<DefeKeyboard>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub message: String,
    pub command: String,
    pub files: Vec<Media>,
    pub keyboards: Vec<Vec<Keyboards>>,
    #[serde(rename = "keyboard_name")]
    pub keyboard_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub file: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefeKeyboard {
    pub name: String,
    pub list: Vec<Vec<Keyboards>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyboards {
    pub text: String,
    pub callback: String,
}

impl Options {
    pub fn get_keyboards(&self) -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        for v in &self.keyboards {
            let mut k: Vec<InlineKeyboardButton> = vec![];
            for x in v {
                k.push(InlineKeyboardButton::callback(
                    x.text.clone(),
                    x.callback.clone(),
                ));
            }
            keyboard.push(k);
        }
        InlineKeyboardMarkup::new(keyboard)
    }
}

static mut JSON_FILE: LazyLock<Arc<RwLock<JSON>>> =
    LazyLock::new(|| Arc::new(RwLock::new(JSON::default())));

impl JSON {
    pub fn get_json() -> Arc<RwLock<JSON>> {
        unsafe { JSON_FILE.clone() }
    }

    pub async fn init_json() {
        match JSON::read("options.json") {
            Ok(mut json) => {
                for opt in json.options.iter_mut() {
                    let key = &opt.keyboard_name;
                    for def in json.keyboards.iter() {
                        if def.name == *key {
                            opt.keyboards = def.list.clone();
                        }
                    }
                }
                unsafe {
                    let mut j = JSON_FILE.write().await;
                    *j = json;
                };
            }
            Err(err) => {
                panic!("Error while reading options file: {}", err);
            }
        };
    }

    pub fn read(path: &str) -> Result<Self, Error> {
        if path.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "path is empty",
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

    pub fn write(&self, path: &str) -> Result<(), Error> {
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
