use mongodb::bson::doc;
use std::sync::Arc;
use twilight_model::{channel::message::MessageFlags, user::User};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::{checks::owner_command, BotState};

#[command]
#[description = "Entfernen einer Meldung eines User (bot owner)"]
#[checks(owner_command)]
#[required_permissions(MANAGE_GUILD)]
pub async fn remove_report(
    ctx: &SlashContext<Arc<BotState>>,
    #[description = "Der User, dem die Meldung entfernt werden soll"] user: User,
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

    let delete_result = ctx
        .data
        .collections
        .reported_users
        .delete_one(doc! { "_id": user.id.to_string() }, None)
        .await?;

    let response = if delete_result.deleted_count == 0 {
        format!("<@{}> war nicht gemeldet!", user.id)
    } else {
        format!("<@{}> ist nun nicht mehr gemeldet!", user.id)
    };

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&response))
        .unwrap()
        .await?;

    Ok(())
}
