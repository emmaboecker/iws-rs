use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use zephyrus::prelude::{command, DefaultCommandResult, SlashContext};

use crate::{
    checks::owner_command,
    database::{IWSCollections, Invitation},
};

#[command]
#[description = "**Bot Owner only** - Invite erstellen"]
#[checks(owner_command)]
pub async fn invite(
    ctx: &SlashContext<Arc<IWSCollections>>,
    #[description = "guild id"] guild_id: String,
) -> DefaultCommandResult {
    ctx.acknowledge().await?;

    let url = create_invite(ctx.data, &guild_id).await?;

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!("Invite: <{}>", url)))
        .unwrap()
        .await?;

    Ok(())
}

pub async fn create_invite(collections: &IWSCollections, guild_id: &str) -> eyre::Result<String> {
    let invite: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    let _ = collections
        .invites
        .insert_one(
            Invitation {
                invite: invite.clone(),
                guild_id: guild_id.parse().unwrap(),
            },
            None,
        )
        .await?;

    let url = url::Url::parse(&format!(
        "{}/invitation/accept/{}",
        std::env::var("WEBSERVER_URL").unwrap(),
        invite
    ))
    .unwrap()
    .to_string();

    Ok(url)
}
