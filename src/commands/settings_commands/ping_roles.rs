use std::sync::Arc;

use mongodb::{bson::doc, options::FindOneAndReplaceOptions};
use twilight_model::{channel::message::MessageFlags, guild::Role};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::{
    checks::only_guilds,
    database::{BotSettings, IWSCollections},
};

#[command]
#[description = "Füge eine Rolle hinzu oder entferne sie von den Rollen die gepingt werden sollen bei einer Meldung"]
#[checks(only_guilds)]
#[required_permissions(MANAGE_GUILD)]
pub async fn ping_roles(
    ctx: &SlashContext<Arc<IWSCollections>>,
    #[description = "Die Rolle die hinzugefügt/entfernt werden soll"] role: Role,
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

    let existing_settings = ctx
        .data
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

    let add = !settings.ping_roles.contains(&role.id);

    let content = if add {
        settings.ping_roles.push(role.id);
        format!("Die Rolle <@&{}> wird nun bei Meldungen gepingt!", role.id)
    } else {
        settings.ping_roles.retain(|r| r != &role.id);
        format!(
            "Die Rolle <@&{}> wird nun nicht mehr bei Meldungen gepingt!",
            role.id
        )
    };

    ctx.data
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
        .content(Some(&content))
        .unwrap()
        .await?;

    Ok(())
}
