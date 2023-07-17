use mongodb::bson::doc;
use twilight_model::{
    channel::message::component::{Button, ButtonStyle::Danger},
    id::Id,
    user::CurrentUserGuild,
};
use zephyrus::twilight_exports::{ActionRow, Component, UserMarker};

use super::report_embed;

pub async fn scan_all_guilds(
    http_client: &twilight_http::Client,
    collections: &crate::database::IWSCollections,
    user: Id<UserMarker>,
) {
    tracing::info!("Scanning all guilds for member {}", user);

    for guild in http_client
        .current_user_guilds()
        .await
        .unwrap()
        .model()
        .await
        .unwrap()
    {
        let result = scan_guild(&guild, http_client, collections, user).await;
        if let Err(report) = result {
            tracing::error!("Failed to scan guild {}: {}", guild.id, report);
        }
    }
}

async fn scan_guild(
    guild: &CurrentUserGuild,
    http_client: &twilight_http::Client,
    collections: &crate::database::IWSCollections,
    user: Id<UserMarker>,
) -> eyre::Result<()> {
    let member = http_client.guild_member(guild.id, user).await;

    if let Err(e) = member {
        if e.to_string().contains("403") {
            return Ok(());
        } else {
            return Err(e.into());
        }
    }

    let member = member.unwrap().model().await?;

    let guild_settings = collections
        .bot_settings
        .find_one(doc! { "_id": guild.id.to_string() }, None)
        .await?;

    let guild_settings = match guild_settings {
        Some(settings) => settings,
        None => return Ok(()),
    };

    if guild_settings.log_channel.is_none() {
        return Ok(());
    }

    let report_info = collections
        .reported_users
        .find_one(doc! { "_id": member.user.id.to_string() }, None)
        .await?;

    if report_info.is_none() {
        return Ok(());
    }

    let report_info = report_info.unwrap();

    let embed_builder = report_embed(
        "Gemeldeter User gefunden!",
        "Es wurde ein auf einem anderen Server gemeldeter User gefunden!",
        &report_info,
        http_client,
        Some(member.user.clone()),
    )
    .await?;

    http_client
        .create_message(guild_settings.log_channel.unwrap())
        .content(
            &guild_settings
                .ping_roles
                .into_iter()
                .map(|role| format!("<@&{}>", role))
                .collect::<Vec<_>>()
                .join(", "),
        )?
        .embeds(&[embed_builder.build()])
        .unwrap()
        .components(&[Component::ActionRow(ActionRow {
            components: vec![Component::Button(Button {
                custom_id: Some(format!("ban:{}", member.user.id)),
                disabled: false,
                emoji: None,
                style: Danger,
                label: Some("User Bannen".to_string()),
                url: None,
            })],
        })])
        .unwrap()
        .await?;

    Ok(())
}
