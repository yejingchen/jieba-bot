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

fn make_inline_result<'a>(
    id: &'a inline_query::Id,
    title: &'a str,
    msg: &'a str
) -> inline_query::Result<'a> {
    let content = Text::new(ParseMode::plain(msg));
    let article = Article::new(title, content).description(msg);
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
            let (title, msg) = if ctx.query.is_empty() {
                ("输入点文字来分词", "就像这样")
            } else {
                ("分词大成功！", &ctx.query[..])
            };
            let answer = JIEBA.cut(msg, false).join(" ");

            let result = make_inline_result(&ctx.id, title, &answer);
            if let Err(e) =  ctx.answer(&[result]).call().await {
                log::warn!("answer InlineQuery failed with: {:?}", e);
            }
        }
    });

    bot.polling().timeout(60_u64).start().await.unwrap();
}
