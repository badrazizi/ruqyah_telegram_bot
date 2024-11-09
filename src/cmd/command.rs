use super::{
    db::is_user_new,
    options::{GetKeybard, Start},
};
use crate::cmd::options::Command;
use std::error::Error;
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
                    let id = user.id.0 as i64;

                    let username = user.username.unwrap_or(user.first_name);
                    let is_user_new = is_user_new(username, id);
                    if is_user_new {
                        bot.send_message(msg.chat.id, "أهلا بك في بوت الرقية الشرعية")
                            .await?;
                    }
                }
                bot.send_message(msg.chat.id, "الاختيارات الرئيسية:")
                    .reply_markup(Start::keyboards())
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
