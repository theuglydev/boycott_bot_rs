mod handlers;
mod models;
use teloxide::{
    dispatching::dialogue::{GetChatId, InMemStorage},
    prelude::*,
    types::InputFile,
};
use url::Url;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveBrand,
}

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("7527659996:AAGVKlDLi13Ml2cd-91oBOPFNJcElh2HI7Y");

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveBrand].endpoint(receive_brand)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        if text.starts_with("/") {
            if text.to_lowercase().contains("start") {
                bot.send_message(msg.chat.id, "Welcome! Let's Start!")
                    .await?;
                bot.send_message(
                    msg.chat.id,
                    "Give me the brand name you want to know about..",
                )
                .await?;

                dialogue.update(State::ReceiveBrand).await?;
            } else {
                bot.send_message(msg.chat.id, "Un-Recognised Command")
                    .await?;
            }
        } else {
            dialogue.update(State::ReceiveBrand).await?;
            let msg_clone = msg.clone();
            let bot_clone = bot.clone();
            let brand_res = receive_brand(bot, dialogue, msg).await;
            if let Err(e) = brand_res {
                bot_clone
                    .send_message(msg_clone.chat.id, e.to_string())
                    .await?;
            }
        }
    }

    Ok(())
}

async fn receive_brand(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        let fetch_res = handlers::brand::get_brands(text.to_string()).await;
        if let Err(e) = fetch_res {
            bot.send_message(msg.chat.id, e.to_string()).await?;
        } else {
            let brands: Vec<models::brand::Brand> = fetch_res.unwrap();
            for brand in brands {
                println!("{}", brand.brand_image);
                let image_url = Url::parse(&brand.brand_image)?;
                let input_file = InputFile::url(image_url);
                let photo_res = bot.send_photo(msg.chat.id, input_file).await;
                if let Err(e) = photo_res {
                    bot.send_message(msg.chat.id, "No Photo for this brand")
                        .await?;
                }

                let text_msg: String = brand.brand_name
                    + &String::from("\n\n")
                    + &brand.proof
                    + &String::from("\n\n")
                    + &String::from("Source: ")
                    + &brand.source;
                bot.send_message(msg.chat.id, text_msg).await?;
            }
        }
    } else {
        bot.send_message(
            msg.chat.id,
            "Give me the brand name you want to know about..",
        )
        .await?;
    }

    dialogue.update(State::Start).await?;

    Ok(())
}
