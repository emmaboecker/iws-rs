use std::sync::Arc;

use mongodb::{bson::doc, options::FindOneAndReplaceOptions};
use twilight_model::{
    channel::{message::MessageFlags, ChannelType},
    id::Id,
};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{ChannelMarker, InteractionResponse, InteractionResponseType},
};

use crate::{commands::error::default_command_error_handler, database::BotSettings, BotState};

#[command]
#[description = "Setzt den Kanal in den Warnungen gesendet werden sollen"]
#[only_guilds]
#[error_handler(default_command_error_handler)]
#[required_permissions(MANAGE_GUILD)]
pub async fn warning_channel(
    ctx: &SlashContext<Arc<BotState>>,
    #[description = "Der Kanal der als neuer Warnungskanal dienen soll"] channel: Id<ChannelMarker>,
) -> DefaultCommandResult {
    ctx.interaction_client
        .create_response(
            ctx.interaction.id,
            &ctx.interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            },
        )
        .await?;

    let channel = ctx.http_client().channel(channel).await?.model().await?;

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

    let existing_settings = ctx
        .data
        .collections
        .bot_settings
        .find_one(
            doc! { "_id": ctx.interaction.guild_id.as_ref().unwrap().to_string() },
            None,
        )
        .await?;

    let mut settings = match existing_settings {
        Some(settings) => settings,
        None => BotSettings {
            guild_id: ctx.interaction.guild_id.unwrap(),
            log_channel: None,
            ping_roles: vec![],
            auto_report: false,
        },
    };

    settings.log_channel = Some(channel.id);

    ctx.data
        .collections
        .bot_settings
        .find_one_and_replace(
            doc! { "_id": ctx.interaction.guild_id.as_ref().unwrap().to_string() },
            settings,
            FindOneAndReplaceOptions::builder()
                .upsert(Some(true))
                .build(),
        )
        .await?;

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!(
            "Der Kanal wurde erfolgreich auf <#{}> gesetzt!",
            channel.id
        )))
        .unwrap()
        .await?;

    Ok(())
}
