use mongodb::bson::doc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::BanAdd;

use crate::{
    database::{BotSettings, IWSCollections, ReportedUser},
    utils::scan_all_guilds,
};

pub async fn ban_add(
    ban_add: BanAdd,
    client: &Client,
    collections: &IWSCollections,
) -> eyre::Result<()> {
    let guild_settings = collections
        .bot_settings
        .find_one(doc! { "_id": ban_add.guild_id.to_string() }, None)
        .await?
        .unwrap_or(BotSettings {
            guild_id: ban_add.guild_id,
            auto_report: false,
            log_channel: None,
            ping_roles: vec![],
        });

    if !guild_settings.auto_report {
        return Ok(());
    }

    let existing_report = collections
        .reported_users
        .find_one(doc! { "_id": ban_add.user.id.to_string() }, None)
        .await?;

    let ban = client
        .ban(ban_add.guild_id, ban_add.user.id)
        .await?
        .model()
        .await?;

    let reason = ban.reason;

    if existing_report.is_some()
        && reason.clone().is_some()
        && !reason
            .clone()
            .unwrap()
            .starts_with("Gebannt durch Button von")
    {
        collections
            .reported_users
            .update_one(
                doc! { "_id": ban_add.user.id.to_string() },
                doc! { "$push": { "reasons": reason.clone().unwrap() } },
                None,
            )
            .await?;
    } else if existing_report.is_none() && reason.is_some() {
        collections
            .reported_users
            .insert_one(
                ReportedUser {
                    discord_id: ban_add.user.id.cast(),
                    reported_on_server: ban_add.guild_id,
                    reported_by: None,
                    reported_at: Some(chrono::Utc::now()),
                    reasons: vec![reason.unwrap()],
                },
                None,
            )
            .await?;
    }

    scan_all_guilds(client, collections, ban_add.user.id).await;

    Ok(())
}
