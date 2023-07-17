use std::sync::Arc;

use mongodb::bson::doc;
use twilight_model::channel::message::MessageFlags;
use twilight_model::user::User;
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::prelude::{command, DefaultCommandResult, SlashContext};
use zephyrus::twilight_exports::{InteractionResponse, InteractionResponseType};

use crate::checks::only_guilds;
use crate::database::IWSCollections;
use crate::utils::report_embed;

#[command]
#[description = "Erhalte Informationen über einen gemeldeten User"]
#[checks(only_guilds)]
#[required_permissions(MANAGE_GUILD)]
pub async fn report_info(
    ctx: &SlashContext<Arc<IWSCollections>>,
    #[description = "Der User über den du die Meldungsinformationen erhalten willst"] user: User,
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

    let report = ctx
        .data
        .reported_users
        .find_one(doc! { "_id": user.id.to_string() }, None)
        .await?;

    if let Some(report) = report {
        let embed_builder = report_embed("Report Informationen", "", &report, ctx.http_client(), None).await?;

        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .embeds(Some(&[embed_builder.build()]))
            .unwrap()
            .await?;

        return Ok(());
    } else {
        ctx.interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some("Dieser User ist nicht gemeldet."))
            .unwrap()
            .await?;
    }

    Ok(())
}
