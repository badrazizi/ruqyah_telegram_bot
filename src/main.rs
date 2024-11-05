use std::env;

use cmd::{callback, command, inline_query};
use teloxide::{dispatching::UpdateFilterExt, dptree, prelude::Dispatcher, types::Update, Bot};
mod cmd;

#[cfg(not(target_os = "windows"))]
#[tokio::main]
async fn main() {
    let _ = init().await;
}

#[cfg(target_os = "windows")]
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
    let bot = Bot::new(env!("TELEGRAM_BOT_TOKEN"));
    // Create a handler for our bot, that will process updates from Telegram
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(command::handler))
        .branch(Update::filter_inline_query().endpoint(inline_query::handler))
        .branch(Update::filter_callback_query().endpoint(callback::handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
