use std::error::Error;
use strum::IntoEnumIterator;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, Me},
    utils::command::BotCommands,
    Bot,
};

use super::{options, selection::make_keyboard};

pub async fn handler(
    bot: Bot,
    q: CallbackQuery,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref text) = q.data {
        println!("command: {text}");
        bot.answer_callback_query(&q.id).await?;
        let id = q.regular_message().unwrap().chat.id;

        match BotCommands::parse(format!("/{text}").as_str(), me.username()) {
            Ok(options::Language::العربية) => {
                let keyboard =
                    make_keyboard(options::RuqyahArabic::iter().map(|ruqyah| ruqyah.to_string()));
                bot.send_message(id, "اختر الرقية الذي تبحث عنها: ")
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(options::Language::English) => {
                bot.send_message(id, "Will be implemented in the near future")
                    .await?;
            }

            Err(_) => {}
        }

        let ruqyah_keyboard =
            make_keyboard(options::RuqyahArabic::iter().map(|ruqyah| ruqyah.to_string()));
        match BotCommands::parse(format!("/{text}").as_str(), me.username()) {
            Ok(options::RuqyahArabic::الرقية_الشرعية) => {
                bot.send_message(
                    id,
                    "السؤال:\nسماحة الشيخ، هذا سائل يقول: ما كيفية الرقية بالدعاء يا سماحة الشيخ؟ وما هي الأدعية التي تقرأ؟\n\nالجواب:\nينفث على المريض على محل المرض، ويدعو له، ينفث عليه من ريقه، ويقرأ الفاتحة، ويكررها سبع مرات، ويقرأ آية الكرسي، ويقرأ ما تيسر من القرآن، ويقرأ: قُلْ هُوَ اللَّهُ أَحَدٌ[الإخلاص:1] والمعوذتين يكررها ثلاثًا، هذه الرقية وينفث معها ويدعو الله، اللهم أذهب الباس رب الناس، واشف أنت الشافي، لا شفاء إلا شفاؤك، شفاءً لا يغادر سقمًا كما فعله النبي ﷺ.\n\nويقول: باسم الله أرقيك من كل شيء يؤذيك، من شر كل نفس، أو عين حاسد، الله يشفيك باسم الله أرقيك هكذا رقى جبرائيل النبي عليه الصلاة والسلام كما أخبر بذلك النبي عليه الصلاة والسلام فكل هذا حسن.\n\nوإذا قال: اللهم اشفه، اللهم عافه، اللهم يسر له العافية والدعوات المناسبة لا بأس، لكن هذا الدعاء الشرعي الوارد عن النبي ﷺ: اللهم أذهب البأس رب الناس، واشف أنت الشافي، لا شفاء إلا شفاؤك، شفاءً لا يغادر سقمًا، باسم الله أرقيك من كل شيء يؤذيك، من شر كل نفس أو عين حاسد الله يشفيك، باسم الله أرقيك، وإذا رقى بدعوات أخرى للمريض بطلب العافية؛ فلا بأس.\n\nمقطع صوتي للشيخ ابن باز (https://files.zadapps.info/binbaz.org.sa/fatawa/nour_3la_aldarb/nour_946/nour_94613.mp3)\n\nالمرجع موقع الشيخ ابن باز (https://binbaz.org.sa/fatwas/18771/%D8%B5%D9%81%D8%A9-%D8%A7%D9%84%D8%B1%D9%82%D9%8A%D8%A9-%D8%A7%D9%84%D8%B4%D8%B1%D8%B9%D9%8A%D8%A9)",
                )
                .await?;

                bot.send_message(id, "اختر الرقية الذي تبحث عنها: ")
                    .reply_markup(ruqyah_keyboard)
                    .await?;
            }
            Ok(options::RuqyahArabic::الرقية_من_العين_والحسد) => {
                bot.send_message(id, "...").await?;

                bot.send_message(id, "اختر الرقية الذي تبحث عنها: ")
                    .reply_markup(ruqyah_keyboard)
                    .await?;
            }

            Err(_) => {}
        }
    }

    Ok(())
}
