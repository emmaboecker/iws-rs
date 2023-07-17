use std::sync::Arc;

use zephyrus::{
    framework::DefaultError,
    prelude::{check, SlashContext},
};

use crate::database::IWSCollections;

#[check]
async fn owner_command(ctx: &SlashContext<Arc<IWSCollections>>) -> Result<bool, DefaultError> {
    let owners = std::env::var("OWNERS")?;

    let runner = ctx
        .interaction
        .member
        .as_ref()
        .unwrap()
        .user
        .as_ref()
        .unwrap();

    Ok(owners
        .split(',')
        .any(|owner| owner == runner.id.to_string()))
}
