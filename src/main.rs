use std::env;

use teloxide::{prelude::*, RequestError};

#[tokio::main]
async fn main() {
    println!("Initializing..");
    let token = &env::args().nth(1).expect("E: Token must be privided as an argument");
    {
        let bot = Bot::new(token);
        println!("started!..");

        teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            repl_handler(&bot, &msg)
                .await
        })
            .await;
    }
}

async fn repl_handler( bot: &Bot, msg: &Message ) -> Result<(), RequestError> {
    let msg_text = msg.text();
    let chat_id = msg.chat.id;
    println!("\nID: {}\nMessage: {:?}\n",chat_id, msg_text);
    bot.send_message(chat_id, "<--->")
        .await?;
    Ok(())
}
