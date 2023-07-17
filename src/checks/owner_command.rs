use std::sync::Arc;

use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    framework::DefaultError,
    prelude::{check, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::database::IWSCollections;

#[check]
async fn owner_command(ctx: &SlashContext<Arc<IWSCollections>>) -> Result<bool, DefaultError> {
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

    let owners = std::env::var("OWNERS")?;
    let support_server = std::env::var("SUPPORT_SERVER")?;

    let runner = ctx
        .interaction
        .member
        .as_ref()
        .unwrap()
        .user
        .as_ref()
        .unwrap();

    let owner_pings = owners
        .split(',')
        .map(|owner| format!("<@{}>", owner))
        .collect::<Vec<_>>()
        .join(", ");

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!("Du darfst diesen Command nur als Bot Owner ausführen. Bitte wende dich an {} bzw. joine diesem Server: {}", owner_pings, support_server)))
        .unwrap()
        .await?;

    Ok(owners
        .split(',')
        .any(|owner| owner == runner.id.to_string()))
}
