use std::{str::FromStr, sync::Arc};

use mongodb::bson::doc;
use twilight_model::{channel::message::MessageFlags, id::Id};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::{checks::owner_command, database::IWSCollections};

#[command]
#[description = "Einen Server entverifizieren (bot owner)"]
#[checks(owner_command)]
#[required_permissions(MANAGE_GUILD)]
pub async fn unverify(
    ctx: &SlashContext<Arc<IWSCollections>>,
    #[description = "guild id"] guild_id: String,
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

    let result = ctx
        .data
        .verified_guilds
        .delete_one(doc! { "_id": guild_id.clone() }, None)
        .await?;

    if result.deleted_count == 0 {
        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("Dieser Server war nicht verifiziert!"))
            .unwrap()
            .await?;
        return Ok(());
    }

    let result = ctx
        .http_client()
        .leave_guild(Id::from_str(&guild_id).unwrap())
        .await;

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!(
            "Der Server {} wurde entverifiziert! {}",
            guild_id,
            if result.is_ok() {
                "Der Server wurde verlassen"
            } else {
                "Der Server konnte aber nicht verlassen werden"
            }
        )))
        .unwrap()
        .await?;

    Ok(())
}
