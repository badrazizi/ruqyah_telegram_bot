use std::error::Error;
use teloxide::{
    prelude::Requester,
    types::{InlineQuery, InlineQueryResultArticle, InputMessageContent, InputMessageContentText},
    Bot,
};

use super::selection::make_keyboard;

pub async fn handler(bot: Bot, q: InlineQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_lanugage = InlineQueryResultArticle::new(
        "0",
        "Chose Language",
        InputMessageContent::Text(InputMessageContentText::new("Language:")),
    )
    .reply_markup(make_keyboard(vec!["".to_string()].into_iter()));

    bot.answer_inline_query(q.id, vec![choose_lanugage.into()])
        .await?;

    Ok(())
}
