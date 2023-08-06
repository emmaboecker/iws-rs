use std::sync::Arc;

use zephyrus::{
    framework::DefaultError,
    prelude::{error_handler, SlashContext},
    twilight_exports::InteractionData,
};

use crate::BotState;

#[error_handler]
pub async fn default_command_error_handler(ctx: &SlashContext<Arc<BotState>>, error: DefaultError) {
    if let Some(InteractionData::ApplicationCommand(data)) = ctx.interaction.data.clone() {
        tracing::error!("The {} command had an error: {:#?}", data.name, error);

        let support_server = std::env::var("SUPPORT_SERVER");

        let result = ctx
            .interaction_client
            .update_response(&ctx.interaction.token)
            .content(Some(&format!(
                "Es ist ein Fehler aufgetreten. Bitte wende dich an den Support: {}",
                support_server.unwrap_or_else(|_| "`Kein Support Server verfügbar`".to_string())
            )))
            .unwrap()
            .await;

        if let Err(e) = result {
            tracing::error!("Error while sending error message: {:#?}", e);
        }
    };
}
