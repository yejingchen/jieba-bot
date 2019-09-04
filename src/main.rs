use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use jieba_rs::Jieba;

fn main() {
    println!("Starting...");
    let jieba = Jieba::new();

    let mut core = Core::new().unwrap();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    println!("Started");

    let future = api.stream().for_each(|update| {

        if let UpdateKind::InlineQuery(query) = update.kind {
            let words = jieba.cut(&query.query, false);
            let joined = words.join(" ");

            let content = InputTextMessageContent {
                message_text: joined.clone(),
                parse_mode: None,
                disable_web_page_preview: true,
            };
            let article = InlineQueryResultArticle::new(&query.query,
                joined, content);
            let result = InlineQueryResult::from(article);

            let answer = AnswerInlineQuery::new(query.id, vec![result]);
            api.spawn(answer);
        }

        Ok(())
    });

    core.run(future).unwrap();
}
