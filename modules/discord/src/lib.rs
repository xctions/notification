pub mod args;

use serenity::{
    all::{CreateEmbed, ExecuteWebhook},
    async_trait,
    model::{channel::Message, gateway::Ready, webhook::Webhook},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn send_embed_message(
    webhook_url: &str,
    embed: CreateEmbed,
) -> Result<(), serenity::Error> {
    let http = serenity::http::Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url)
        .await
        .expect("Could not get webhook");

    let builder = ExecuteWebhook::new()
        .embeds(vec![embed])
        .username("Webhook test");

    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");

    Ok(())
}
