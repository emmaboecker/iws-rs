use std::sync::Arc;

use zephyrus::{
    framework::DefaultError,
    prelude::{check, SlashContext},
};

use crate::database::IWSCollections;

#[check]
async fn only_guilds(ctx: &SlashContext<Arc<IWSCollections>>) -> Result<bool, DefaultError> {
    Ok(ctx.interaction.guild_id.is_some())
}
