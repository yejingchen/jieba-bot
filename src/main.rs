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

fn make_answer<'a>(id: &'a inline_query::Id, msg: &'a str)
-> inline_query::Result<'a> {
    let content = Text::new(ParseMode::plain(msg));
    let article = Article::new(msg, content);
    let result = inline_query::Result::new(&id.0, article);
    return result;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("jieba-bot: Chinese word segmentation Telegram bot");

    // BOT_TOKEN is a compile time environment variable
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.inline(|ctx| {
        async move {
            let answer = if ctx.query.is_empty() {
                "输入点文字来分词".to_owned()
            } else {
                JIEBA.cut(&ctx.query, false).join(" ")
            };

            let result = make_answer(&ctx.id, &answer);
            if let Err(e) =  ctx.answer(&[result]).call().await {
                log::warn!("answer InlineQuery failed with: {:?}", e);
            }
        }
    });

    bot.polling().timeout(60).start().await.unwrap();
}
