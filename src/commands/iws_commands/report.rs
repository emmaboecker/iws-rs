use std::sync::Arc;

use mongodb::bson::doc;
use twilight_model::channel::message::MessageFlags;
use twilight_model::user::User;
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::prelude::{command, DefaultCommandResult, SlashContext};
use zephyrus::twilight_exports::{InteractionResponse, InteractionResponseType};

use crate::commands::error::default_command_error_handler;
use crate::database::ReportedUser;
use crate::utils::scan_all_guilds;
use crate::BotState;

#[command]
#[description = "Einen User melden"]
#[only_guilds]
#[error_handler(default_command_error_handler)]
#[required_permissions(MANAGE_GUILD)]
pub async fn report(
    ctx: &SlashContext<Arc<BotState>>,
    #[description = "Der User der gemeldet werden soll"] user: User,
    #[description = "Wofür dieser User gemeldet werden soll"] reason: String,
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

    if user.bot {
        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("Bots können nicht gemeldet werden!"))
            .unwrap()
            .await?;
        return Ok(());
    }

    let existing_report = ctx
        .data
        .collections
        .reported_users
        .find_one(doc! { "_id": user.id.to_string() }, None)
        .await?;

    if let Some(existing_report) = existing_report {
        let content = format!(
            "Dieser User wurde bereits {} gemeldet! Dein Grund wurde aber hinterlegt",
            if !existing_report.reasons.is_empty() {
                format!(
                    "für {}",
                    existing_report
                        .reasons
                        .into_iter()
                        .map(|reason| format!("`{}`", reason))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                "".to_string()
            }
        );

        ctx.data
            .collections
            .reported_users
            .update_one(
                doc! { "_id": user.id.to_string() },
                doc! { "$push": { "reasons": reason } },
                None,
            )
            .await?;

        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some(&content))
            .unwrap()
            .await?;
        return Ok(());
    } else {
        ctx.data
            .collections
            .reported_users
            .insert_one(
                ReportedUser {
                    discord_id: user.id.cast(),
                    reasons: vec![reason],
                    reported_at: Some(chrono::Utc::now()),
                    reported_by: Some(
                        ctx.interaction
                            .member
                            .as_ref()
                            .unwrap()
                            .user
                            .as_ref()
                            .unwrap()
                            .id
                            .cast(),
                    ),
                    reported_on_server: ctx.interaction.guild_id.unwrap(),
                },
                None,
            )
            .await?;

        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("User wurde gemeldet!"))
            .unwrap()
            .await?;
    }

    scan_all_guilds(ctx.http_client(), &ctx.data.collections, user.id).await;

    Ok(())
}
