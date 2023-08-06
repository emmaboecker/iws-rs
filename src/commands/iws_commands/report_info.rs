use std::sync::Arc;

use mongodb::bson::doc;

use twilight_model::user::User;

use zephyrus::prelude::{command, DefaultCommandResult, SlashContext};

use crate::commands::error::default_command_error_handler;
use crate::utils::report_embed;
use crate::BotState;

#[command]
#[description = "Erhalte Informationen über einen gemeldeten User"]
#[only_guilds]
#[required_permissions(MANAGE_GUILD)]
#[error_handler(default_command_error_handler)]
pub async fn report_info(
    ctx: &SlashContext<Arc<BotState>>,
    #[description = "Der User über den du die Meldungsinformationen erhalten willst"] user: User,
) -> DefaultCommandResult {
    ctx.defer(true).await?;

    let report = ctx
        .data
        .collections
        .reported_users
        .find_one(doc! { "_id": user.id.to_string() }, None)
        .await?;

    if let Some(report) = report {
        let embed_builder =
            report_embed("Report Informationen", "", &report, ctx.http_client(), None).await?;

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
