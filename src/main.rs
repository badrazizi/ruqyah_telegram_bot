use cmd::{callback, command, json::JSON};
use config::file::Configurations;
use database::{table::Table, users::Users};
use notify::{Event, RecursiveMode, Watcher};
use rusqlite::Connection;
use std::{path::Path, sync::mpsc, time::Duration};
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Dispatcher,
    types::{CallbackQuery, Me, Message, Update},
    Bot,
};
use tokio::{task, time};
mod cmd;
mod config;
mod database;

#[cfg(not(debug_assertions))]
#[tokio::main]
async fn main() {
    let _ = init().await;
}

#[cfg(debug_assertions)]
fn main() {
    let child = std::thread::Builder::new()
        .stack_size(4096 * 1024)
        .spawn(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let _ = init().await;
                });
        })
        .unwrap();
    child.join().unwrap();
}

async fn init() -> Result<(), Box<dyn std::error::Error>> {
    init_tables()?;

    JSON::init_json().await;

    task::spawn(file_changed_notify());

    let config = Configurations::get_config();

    if !config.telegram_token.is_empty() {
        start_bot(config.telegram_token.clone()).await?;
    } else {
        panic!("telegram bot token not set");
    }

    Ok(())
}

async fn start_bot(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::new(token);

    // Create a handler for our bot, that will process updates from Telegram
    let handler = dptree::entry()
        .branch(
            Update::filter_message().endpoint(|bot: Bot, msg: Message, me: Me| async move {
                command::handler(bot, msg, me).await
            }),
        )
        .branch(Update::filter_callback_query().endpoint(
            |bot: Bot, q: CallbackQuery, me: Me| async move { callback::handler(bot, q, me).await },
        ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

fn init_tables() -> Result<(), Box<dyn std::error::Error>> {
    println!("Database opened successfully");
    let connection: Connection = match Connection::open("./database.db") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while opening database",
            )));
        }
    };

    match Users::init_table(&connection) {
        Ok(_) => {}
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while creating Users table",
            )));
        }
    }

    Ok(())
}

async fn file_changed_notify() {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    if let Ok(mut watcher) = notify::recommended_watcher(tx) {
        if let Ok(_) = watcher.watch(Path::new("."), RecursiveMode::NonRecursive) {
            for res in rx {
                if let Ok(event) = res {
                    if event.kind.is_modify() {
                        for path in event.paths.iter() {
                            if path.is_file() {
                                if let Some(file_name) = path.file_name() {
                                    if file_name == "options.json" {
                                        time::sleep(Duration::from_millis(2000)).await;
                                        JSON::init_json().await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
