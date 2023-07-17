#![allow(clippy::unused_unit)]

use ::mongodb::{options::ClientOptions, Client as MongoClient};
use commands::VerificationCommands;
use database::IWSCollections;
use dotenvy::dotenv;
use futures::StreamExt;
use listeners::process_event;
use std::env;
use std::sync::Arc;
use twilight_gateway::stream::{self, ShardEventStream};
use twilight_gateway::Config;
use twilight_http::Client;
use twilight_model::gateway::Intents;
use twilight_model::id::marker::{ApplicationMarker, GuildMarker};
use twilight_model::id::Id;
use zephyrus::prelude::*;

pub mod commands;
pub mod database;

mod listeners;

pub mod checks;

pub mod http_server;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN")?;
    let app_id = Id::<ApplicationMarker>::new(env::var("APP_ID")?.parse()?);

    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set!");
    let mongo_database = env::var("MONGO_DATABASE").expect("MONGO_DATABASE not set!");
    let mongo_client_options = ClientOptions::parse(mongo_url).await?;
    let mongo_client = MongoClient::with_options(mongo_client_options)?;
    let mongo_database = mongo_client.database(&mongo_database);

    let collections = IWSCollections {
        reported_users: mongo_database.collection("banned_users"),
        bot_settings: mongo_database.collection("ban_settings"),
        scan_cooldown: mongo_database.collection("scans"),
        invites: mongo_database.collection("invites"),
        verified_guilds: mongo_database.collection("verified_guilds"),
    };

    let client = Arc::new(Client::builder().token(token.clone()).build());
    let config = Config::new(
        token,
        Intents::GUILDS | Intents::GUILD_MEMBERS | Intents::GUILD_MODERATION,
    );

    let mut shards = stream::create_recommended(&client, config, |_, builder| builder.build())
        .await?
        .collect::<Vec<_>>();

    let stream = ShardEventStream::new(shards.iter_mut());

    let collections = Arc::new(collections);

    let framework = Arc::new(
        Framework::builder(client.clone(), app_id, collections.clone())
            .verification_commands()
            .build(),
    );

    if let Ok(test_server) = std::env::var("TEST_SERVER") {
        let result = framework
            .register_guild_commands(Id::<GuildMarker>::new(
                test_server
                    .parse()
                    .expect("Invalid guild ID for TEST_SERVER"),
            ))
            .await;

        if let Err(source) = result {
            tracing::error!(?source, "failed to register commands");
        }
    } else {
        framework.register_global_commands().await.unwrap();
    }

    {
        let collections = collections.clone();
        tokio::spawn(async move {
            http_server::http_server(collections).await.unwrap();
        });
    }

    handle_events(stream, client, collections.clone(), framework).await;

    Ok(())
}

async fn handle_events(
    mut events: ShardEventStream<'_>,
    http_client: Arc<Client>,
    collections: Arc<IWSCollections>,
    framework: Arc<Framework<Arc<IWSCollections>>>,
) {
    while let Some((_, event)) = events.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn(process_event(
            event,
            http_client.clone(),
            collections.clone(),
            framework.clone(),
        ));
    }
}
