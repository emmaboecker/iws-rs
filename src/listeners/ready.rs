use mongodb::bson::doc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::Ready;

use crate::{commands::create_invite, database::IWSCollections};

pub async fn ready(ready: Box<Ready>, client: &Client, collections: &IWSCollections) {
    tracing::info!("Ready! Logged in as {}", ready.user.name);

    let guilds = client
        .current_user_guilds()
        .await
        .unwrap()
        .model()
        .await
        .unwrap();

    let mut new_guilds = guilds.clone();

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    for guild in guilds {
        tracing::info!(
            "Checking guild for verification: {} ({})",
            guild.name,
            guild.id
        );
        if collections
            .verified_guilds
            .find_one(doc! { "_id": guild.id.to_string(), "verified": true }, None)
            .await
            .unwrap()
            .is_none()
        {
            let leave_guilds =
                std::env::var("LEAVE_UNVERIFIED").unwrap_or("false".to_string()) == "true";

            if leave_guilds {
                client.leave_guild(guild.id).await.unwrap_or_else(|_| {
                    panic!(
                        "Failed to leave guild without verification: {} ({})",
                        guild.name, guild.id
                    )
                });

                new_guilds.retain(|current| current.id != guild.id);

                tracing::info!(
                    "Left guild without verification: {} ({})",
                    guild.name,
                    guild.id
                );
            } else {
                tracing::warn!(
                    "Bot is on unverified guild but leaving was disabled: {} ({})",
                    guild.name,
                    guild.id
                );
            }
        } else {
            tracing::info!("Checked verified guild: {} ({})", guild.name, guild.id);
        }
    }

    if new_guilds.is_empty() {
        let guild_id = std::env::var("INITIAL_GUILD");

        if let Ok(guild_id) = guild_id {
            let invite = create_invite(collections, &guild_id)
                .await
                .expect("Failed to create invite");

            tracing::warn!(
                "The Bot is not on any guild. Use this invite for {}: {}",
                guild_id,
                invite
            );
        } else {
            tracing::warn!("The bot is not on any guild. Set the INITIAL_GUILD environment variable to the ID of a guild to create an invite for it.")
        }
    }
}
