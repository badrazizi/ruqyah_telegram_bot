use super::selection::make_keyboard;
use crate::cmd::options::{Command, Language};
use std::error::Error;
use strum::IntoEnumIterator;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Me, Message},
    utils::command::BotCommands,
    Bot,
};

pub async fn handler(bot: Bot, msg: Message, me: Me) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                if let Some(user) = msg.from {
                    let username = user.clone().username.unwrap_or_else(|| user.full_name());
                    bot.send_message(
                        msg.chat.id,
                        format!("أهلا {} بك في بوت الرقية الشرعية", username),
                    )
                    .await?;
                }

                let keyboard = make_keyboard(Language::iter().map(|lang| lang.to_string()));
                bot.send_message(
                    msg.chat.id,
                    "الرجاء إختيار اللغة:\nPlease choose a language:",
                )
                .reply_markup(keyboard)
                .await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "آمر غير معروف، للبدء أرسل\n/start")
                    .await?;
            }
        }
    }

    Ok(())
}
