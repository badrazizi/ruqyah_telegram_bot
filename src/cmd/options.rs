use teloxide::utils::command::BotCommands;

pub trait OptionsPair {
    fn get_options() -> Vec<(String, String)>;
}

#[derive(BotCommands)]
#[command(rename_rule = "PascalCase")]
pub enum Command {
    Start,
}

impl OptionsPair for Command {
    fn get_options() -> Vec<(String, String)> {
        vec![("أبداء".to_string(), "/Start".to_string())]
    }
}

#[derive(BotCommands)]
#[command(rename_rule = "PascalCase")]
pub enum Start {
    Ruqyah,
    AboutRuqyah,
    RulingRuqyah,
    VirtueOfRuqyah,
    AboutBot,
}

impl OptionsPair for Start {
    fn get_options() -> Vec<(String, String)> {
        vec![
            ("الرقية الشرعية".to_string(), "/Ruqyah".to_string()),
            ("عن الرقية الشرعية".to_string(), "/AboutRuqyah".to_string()),
            (
                "حكم الرقية الشرعية".to_string(),
                "/RulingRuqyah".to_string(),
            ),
            (
                "فضل الرقية الشرعية".to_string(),
                "/VirtueOfRuqyah".to_string(),
            ),
            ("عن البوت".to_string(), "/AboutBot".to_string()),
        ]
    }
}

#[derive(BotCommands)]
#[command(rename_rule = "PascalCase")]
pub enum Ruqyah {
    RuqyahFromTheHolyQuran,
    RuqyahFromTheNoblePropheticSunnah,
}

impl OptionsPair for Ruqyah {
    fn get_options() -> Vec<(String, String)> {
        vec![
            (
                "الرقية الشرعية من القرآن الكريم".to_string(),
                "/RuqyahFromTheHolyQuran".to_string(),
            ),
            (
                "الرقية الشرعية من السنة النبوية الشريفة".to_string(),
                "/RuqyahFromTheNoblePropheticSunnah".to_string(),
            ),
        ]
    }
}
