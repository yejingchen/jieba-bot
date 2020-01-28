use lazy_static::lazy_static;
use jieba_rs::Jieba;
use tbot::types::{
    inline_query::{self, result::Article},
    input_message_content::Text,
    parameters::Text as ParseMode,
};

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("jieba-bot: Chinese word segmentation Telegram bot");

    // BOT_TOKEN is a compile time environment variable
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.inline(move |ctx| {
        async move {
            let words = JIEBA.cut(&ctx.query, false);
            let joined = words.join(" ");

            let content = Text::new(ParseMode::plain(&joined));
            let article = Article::new(&joined, content);
            let result = inline_query::Result::new(&ctx.id.0, article);
            if let Err(e) =  ctx.answer(&[result]).call().await {
                log::warn!("answer InlineQuery failed with: {:?}", e);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
