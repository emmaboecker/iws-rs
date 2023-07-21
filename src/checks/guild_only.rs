use std::sync::Arc;

use zephyrus::{
    framework::DefaultError,
    prelude::{check, SlashContext},
};

use crate::BotState;

#[check]
async fn only_guilds(ctx: &SlashContext<Arc<BotState>>) -> Result<bool, DefaultError> {
    Ok(ctx.interaction.guild_id.is_some())
}
