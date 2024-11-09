use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

pub trait GetKeybard {
    fn keyboards() -> InlineKeyboardMarkup;
}

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
}

impl GetKeybard for Command {
    fn keyboards() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

        keyboard.push(vec![InlineKeyboardButton::callback(
            "أبداء".to_string(),
            "/start".to_string(),
        )]);

        InlineKeyboardMarkup::new(keyboard)
    }
}

#[derive(BotCommands)]
#[command(rename_rule = "PascalCase")]
pub enum Start {
    Ruqyah,
    AboutRuqyah,
    AboutBot,
}

impl GetKeybard for Start {
    fn keyboards() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        keyboard.push(vec![InlineKeyboardButton::callback(
            "الرقية الشرعية".to_string(),
            "/Ruqyah".to_string(),
        )]);

        keyboard.push(vec![InlineKeyboardButton::callback(
            "عن الرقية الشرعية".to_string(),
            "/AboutRuqyah".to_string(),
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            "عن البوت".to_string(),
            "/AboutBot".to_string(),
        )]);

        InlineKeyboardMarkup::new(keyboard)
    }
}

#[derive(BotCommands)]
#[command(rename_rule = "PascalCase")]
pub enum Ruqyah {
    RuqyahFromTheHolyQuran,
    RuqyahFromTheNoblePropheticSunnah,
    VideoSheikhWahidBali,
    RuqyahForTheLoverDemon,
    RuqyahForKids,
    PrayerToBreakTheSpell,
    Back1,
}

impl GetKeybard for Ruqyah {
    fn keyboards() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
        keyboard.push(vec![InlineKeyboardButton::callback(
            "الرقية الشرعية من القرآن الكريم".to_string(),
            "/RuqyahFromTheHolyQuran".to_string(),
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            "الرقية الشرعية من السنة النبوية الشريفة".to_string(),
            "/RuqyahFromTheNoblePropheticSunnah".to_string(),
        )]);

        keyboard.push(vec![InlineKeyboardButton::callback(
            "علاج العين والحسد للشيخ وحيد بالي".to_string(),
            "/VideoSheikhWahidBali".to_string(),
        )]);

        keyboard.push(vec![
            InlineKeyboardButton::callback(
                "رقية المس العاشق".to_string(),
                "/RuqyahForTheLoverDemon".to_string(),
            ),
            InlineKeyboardButton::callback(
                "رقية الاطفال".to_string(),
                "/RuqyahForKids".to_string(),
            ),
        ]);

        keyboard.push(vec![InlineKeyboardButton::callback(
            "دعاء لفك السحر".to_string(),
            "/PrayerToBreakTheSpell".to_string(),
        )]);

        keyboard.push(vec![InlineKeyboardButton::callback(
            "رجوع".to_string(),
            "/Back1".to_string(),
        )]);

        InlineKeyboardMarkup::new(keyboard)
    }
}
