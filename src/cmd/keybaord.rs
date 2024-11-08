use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_keyboard(iter: Vec<String>, chunk_size: i8) -> InlineKeyboardMarkup {
    let mut chunks = chunk_size;
    if chunks > 3 {
        chunks = 3;
    }

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for items in iter.chunks(chunks as usize) {
        let row = items
            .iter()
            .map(|item| InlineKeyboardButton::callback(item.replace("_", " "), item))
            .collect();
        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
