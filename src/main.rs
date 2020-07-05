use hyper_proxy::{Proxy, Intercept};
use lazy_static::lazy_static;
use jieba_rs::Jieba;
use tbot::types::{
    inline_query::{self, result::Article},
    input_message_content::Text,
    parameters::Text as ParseMode,
};

const BOT_TOKEN_ENV: &str = "BOT_TOKEN";

lazy_static! {
    static ref JIEBA: Jieba = Jieba::new();
}

fn make_inline_result<'a>(id: &'a str, title: &'a str, msg: &'a str)
-> inline_query::Result<'a> {
    let content = Text::new(ParseMode::plain(msg));
    let article = Article::new(title, content).description(msg);
    let result = inline_query::Result::new(id, article);
    return result;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("jieba-bot: Chinese word segmentation Telegram bot");

    let mut bot = match std::env::var("HTTPS_PROXY") {
        Ok(proxy_uri) => {
            log::info!("Using https proxy {}", proxy_uri);
            let proxy_uri = proxy_uri.parse().unwrap();
            let proxy = Proxy::new(Intercept::Https, proxy_uri);

            tbot::Bot::from_env_with_proxy(BOT_TOKEN_ENV, proxy).event_loop()
        },
        Err(e) => {
            log::info!("Not using proxy because {}", e);

            tbot::Bot::from_env(BOT_TOKEN_ENV).event_loop()
        }
    };

    bot.inline(|ctx| {
        async move {
            if ctx.query.is_empty() {
                let result = make_inline_result(&ctx.id.0, "输入点文字来分词", "就 会 像 这样");
                if let Err(e) = ctx.answer(&[result]).call().await {
                    log::warn!("Default message (id={}) failed with: {}", &ctx.id.0, e);
                }
            } else {
                let answer = JIEBA.cut(&ctx.query, false).join(" ");
                let result = make_inline_result(&ctx.id.0, "分词大成功！", &answer);

                let hmm_answer = JIEBA.cut(&ctx.query, true).join(" ");
                let hmm_id = format!("{}_hmm", ctx.id.0);
                let hmm_result = make_inline_result(&hmm_id, "马尔可夫 ON！", &hmm_answer);

                if let Err(e) = ctx.answer(&[result, hmm_result]).call().await {
                    log::warn!("Answering InlineQuery (id={}) failed with: {}", &ctx.id.0, e);
                }
            }
        }
    });

    bot.polling().timeout(60_u64).start().await.unwrap();
}
