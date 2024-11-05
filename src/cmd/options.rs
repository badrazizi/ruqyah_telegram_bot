use strum_macros::{Display, EnumIter};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Display)]
#[command(rename_rule = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Command {
    Start,
}

#[derive(BotCommands, Display, EnumIter)]
#[command(rename_rule = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    العربية,
    English,
}

#[derive(BotCommands, Display, EnumIter)]
#[command(rename_rule = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum RuqyahArabic {
    الرقية_الشرعية,
    الرقية_من_العين_والحسد,
}
