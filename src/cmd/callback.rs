use super::json::JSON;
use crate::config::file::Configurations;
use infer::MatcherType;
use std::{error::Error, path::PathBuf};
use teloxide::{
    payloads::{
        SendAudioSetters, SendDocumentSetters, SendMessageSetters, SendPhotoSetters,
        SendVideoSetters,
    },
    prelude::Requester,
    types::{CallbackQuery, InputFile, Me},
    Bot,
};

pub async fn handler(
    bot: Bot,
    q: CallbackQuery,
    _: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = q.regular_message().unwrap().chat.id;

    if let Some(ref text) = q.data {
        let c = Configurations::get_config();
        let json_lock = JSON::get_json();
        let j = json_lock.read().await;

        for item in j.options.iter() {
            if item.command == *text {
                if item.files.len() > 0 {
                    for media in &item.files {
                        let base_path = PathBuf::from(c.files_base_dir.clone());
                        let file_path = base_path.join(media.file.clone());
                        match infer::get_from_path(&file_path) {
                            Ok(v) => match v {
                                Some(a) => match a.matcher_type() {
                                    MatcherType::Image => {
                                        let f = InputFile::file(file_path);
                                        bot.send_photo(chat_id, f)
                                            .caption(media.caption.clone())
                                            .await?;
                                    }
                                    MatcherType::Audio => {
                                        let f = InputFile::file(file_path);
                                        bot.send_audio(chat_id, f)
                                            .caption(media.caption.clone())
                                            .await?;
                                    }
                                    MatcherType::Video => {
                                        let f = InputFile::file(file_path);
                                        bot.send_video(chat_id, f)
                                            .caption(media.caption.clone())
                                            .await?;
                                    }
                                    MatcherType::Doc => {
                                        let f = InputFile::file(file_path);
                                        bot.send_document(chat_id, f)
                                            .caption(media.caption.clone())
                                            .await?;
                                    }
                                    _ => {}
                                },
                                None => {}
                            },
                            Err(_) => {}
                        };
                    }
                }

                if item.message.len() > 0 {
                    bot.send_message(chat_id, item.message.clone())
                        .reply_markup(item.get_keyboards())
                        .await?;
                }

                bot.answer_callback_query(&q.id).await?;
                return Ok(());
            }
        }
    }

    bot.answer_callback_query(&q.id).await?;
    bot.send_message(chat_id, "آمر غير معروف، للبدء أرسل\n/start")
        .await?;
    return Err("Command not found".into());
}
