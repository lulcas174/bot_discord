mod cep_request;

use std::env;

use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use dotenv::dotenv;

use crate::cep_request::get_cep_info;


#[group]
#[commands(ping, localizacao)]
struct General; 

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main(){
    dotenv().ok();
   /*
        Declaramos a variável que recebe o standard framework, que é o framework padrão do serenity
        e configuramos o prefixo do bot para "!".
        Depois, declaramos a variável que recebe o token do bot, que está na variável de ambiente
    */
    let framework = StandardFramework::new()
    .configure(|c| c.prefix("!")) 
    .group(&GENERAL_GROUP);
    
    // Cria a variável que recebe o token do BOT
    let token = env::var("DISCORD_TOKEN").expect("token");
    // Cria a variável que recebe as intents do bot
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    // Cria a variável que recebe o client do bot e passa o token e as intents
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let user_message = msg.content.clone();
    msg.reply(ctx, "Pong!").await?;
    msg.reply(ctx, user_message).await?;
    Ok(())
}

#[command]
async fn localizacao(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_message = args.rest();
    if user_message.len() != 8 || !user_message.chars().all(char::is_numeric) || user_message.is_empty() {
        msg.reply(ctx, "CEP inválido").await?;
        return Ok(());
    }
    let api_return = get_cep_info(user_message.to_string()).await?;
    msg.reply(ctx, api_return).await?;
    Ok(())
}