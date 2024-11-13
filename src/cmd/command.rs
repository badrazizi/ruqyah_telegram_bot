use super::json::{Options, JSON};
use crate::database;
use std::error::Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Me, Message},
    Bot,
};

pub async fn handler(bot: Bot, msg: Message, _: Me) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        let json_lock = JSON::get_json();
        let json = json_lock.read().await;

        if text.to_lowercase() == "/start" {
            if let Some(user) = msg.from {
                let id = user.id.0 as i64;

                let username = user.username.unwrap_or(user.first_name);
                let is_user_new = database::users::Users::is_user_new(username, id);
                if is_user_new {
                    bot.send_message(msg.chat.id, json.welcome.clone()).await?;
                }
            }

            match json
                .options
                .iter()
                .filter(|v| v.command.contains(&String::from("/start")))
                .collect::<Vec<&Options>>()
                .first()
            {
                Some(v) => {
                    bot.send_message(msg.chat.id, v.message.clone())
                        .reply_markup(v.get_keyboards())
                        .await?;
                }
                None => {}
            }
        } else {
            bot.send_message(msg.chat.id, "آمر غير معروف، للبدء أرسل\n/start")
                .await?;
        }
    }

    Ok(())
}
