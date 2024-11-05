use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_keyboard<I>(iter: I) -> InlineKeyboardMarkup
where
    I: Iterator<Item = String>,
{
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for language in iter {
        let row = vec![InlineKeyboardButton::callback(
            language.replace("_", " "),
            language,
        )];
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
