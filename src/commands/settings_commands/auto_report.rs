use std::sync::Arc;

use mongodb::{bson::doc, options::FindOneAndReplaceOptions};
use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::{checks::only_guilds, database::BotSettings, BotState};

#[command]
#[description = "Toggle ob Bans automatisch gemeldet werden sollen"]
#[checks(only_guilds)]
#[required_permissions(MANAGE_GUILD)]
pub async fn auto_report(ctx: &SlashContext<Arc<BotState>>) -> DefaultCommandResult {
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

    let content = if settings.auto_report {
        settings.auto_report = false;
        "Auto Report wurde deaktiviert!"
    } else {
        settings.auto_report = true;
        "Auto Report wurde aktiviert!"
    };

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
        .content(Some(content))
        .unwrap()
        .await?;

    Ok(())
}
