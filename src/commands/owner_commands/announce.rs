use std::sync::Arc;

use futures::StreamExt;
use mongodb::bson::doc;
use zephyrus::prelude::{command, DefaultCommandResult, Modal, SlashContext};

use crate::{checks::owner_command, commands::error::default_command_error_handler, BotState};

#[derive(Modal, Debug)]
#[modal(title = "Bot Announcement")]
struct AnnouncementModal {
    #[modal(paragraph, label = "Paragraph")]
    pub paragraph: String,
}

#[command]
#[description = "Ein Announcement an alle Server senden (bot owner)"]
#[checks(owner_command)]
#[error_handler(default_command_error_handler)]
#[required_permissions(MANAGE_GUILD)]
pub async fn announce(ctx: &SlashContext<Arc<BotState>>) -> DefaultCommandResult {
    let modal_waiter = ctx.create_modal::<AnnouncementModal>().await?;
    let output = modal_waiter.await?;

    let all_set_guilds = ctx
        .data
        .collections
        .bot_settings
        .find(doc! {}, None)
        .await?
        .collect::<Vec<_>>()
        .await;

    for guild in all_set_guilds {
        let guild = guild?;
        if guild.log_channel.is_none() {
            continue;
        }

        let _ = ctx
            .http_client()
            .create_message(guild.log_channel.unwrap())
            .content(&format!(
                "# IWS Announcement\n{}\n\n{}",
                guild
                    .ping_roles
                    .into_iter()
                    .map(|role| format!("<@&{}>", role))
                    .collect::<Vec<_>>()
                    .join(", "),
                output.paragraph
            ))?
            .await?;
    }

    Ok(())
}
