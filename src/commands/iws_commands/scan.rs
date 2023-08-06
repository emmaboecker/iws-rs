use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOneAndReplaceOptions;
use std::sync::Arc;
use twilight_model::channel::ChannelType;
use twilight_model::id::Id;
use twilight_util::builder::embed::EmbedBuilder;
use zephyrus::prelude::{command, DefaultCommandResult, SlashContext};
use zephyrus::twilight_exports::ChannelMarker;

use crate::commands::error::default_command_error_handler;
use crate::database::ScanCooldown;
use crate::BotState;

#[command]
#[description = "Scanne diesen Server nach gemeldeten Usern (24h cooldown)"]
#[only_guilds]
#[error_handler(default_command_error_handler)]
#[required_permissions(MANAGE_GUILD)]
pub async fn scan(
    ctx: &SlashContext<Arc<BotState>>,
    #[description = "Der Kanal in den der Report gesendet werden soll, standartmäßig aktueller kanal"]
    channel: Option<Id<ChannelMarker>>,
) -> DefaultCommandResult {
    ctx.defer(false).await?;

    let channel = if let Some(channel) = channel {
        ctx.http_client().channel(channel).await?.model().await?
    } else {
        ctx.interaction.channel.clone().unwrap()
    };

    if channel.guild_id != Some(ctx.interaction.guild_id.unwrap()) {
        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("Dieser Kanal ist nicht auf diesem Server!"))
            .unwrap()
            .await?;
        return Ok(());
    }

    if channel.kind != ChannelType::GuildText {
        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("Dieser Kanal ist kein Textkanal!"))
            .unwrap()
            .await?;
        return Ok(());
    }

    ctx.data
        .collections
        .scan_cooldown
        .find_one_and_replace(
            doc! { "_id": ctx.interaction.guild_id.unwrap().to_string() },
            ScanCooldown {
                guild_id: ctx.interaction.guild_id.unwrap(),
                last_scan: chrono::Utc::now(),
            },
            FindOneAndReplaceOptions::builder().upsert(true).build(),
        )
        .await?;

    let message = ctx
        .http_client()
        .create_message(channel.id)
        .embeds(&[EmbedBuilder::new()
            .title("Scan Report")
            .description("Suche nach gemeldeten Mitgliedern...")
            .build()])?
        .await?
        .model()
        .await?;

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some("Scan gestartet."))
        .unwrap()
        .await?;

    let reports = ctx
        .data
        .collections
        .reported_users
        .find(doc! {}, None)
        .await?
        .collect::<Vec<_>>()
        .await;

    let mut found_members = vec![];

    let mut last_id = Id::new(1);

    loop {
        let members = ctx
            .http_client()
            .guild_members(ctx.interaction.guild_id.unwrap())
            .after(last_id)
            .limit(1000)?
            .await?
            .model()
            .await?;

        for member in &members {
            if reports
                .iter()
                .any(|report| report.as_ref().unwrap().discord_id == member.user.id)
            {
                found_members.push(member.clone());
            }
        }

        last_id = members.last().unwrap().user.id;

        if members.len() < 1000 {
            break;
        }
    }

    ctx.http_client()
        .update_message(channel.id, message.id)
        .embeds(Some(&[EmbedBuilder::new()
            .title("Scan Report")
            .description(format!(
                "Es wurde(n) {} gemeldete(s) Mitglied(er) gefunden.\n{}",
                found_members.len(),
                found_members
                    .iter()
                    .map(|member| {
                        format!(
                            "<@{}> ({} / {})",
                            member.user.id, member.user.name, member.user.id
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
            .build()]))?
        .await?;

    Ok(())
}
