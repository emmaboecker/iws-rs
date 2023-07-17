use zephyrus::{
    framework::DefaultError,
    prelude::{check, SlashContext},
};

#[check]
async fn only_guilds(ctx: &SlashContext<()>) -> Result<bool, DefaultError> {
    Ok(ctx.interaction.guild_id.is_some())
}
