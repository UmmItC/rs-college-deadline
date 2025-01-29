use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Messages {
    commands: CommandResponses,
}

#[derive(Debug, Deserialize)]
struct CommandResponses {
    gay: String,
    about: String,
}

struct Bot {
    messages: Messages,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!about" {
            if let Err(e) = msg.channel_id.say(&ctx.http, &self.messages.commands.about).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {

    // Get messages from Secrets.toml
    let messages_toml = secrets
        .get("MESSAGES_TOML")
        .context("'MESSAGES_TOML' was not found")?;
        
    let messages: Messages = toml::from_str(&messages_toml)
        .context("Failed to parse messages TOML from secrets")?;

    // Get the discord token
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { messages })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
