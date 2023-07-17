use std::sync::Arc;

use twilight_gateway::Event;
use twilight_http::Client;
use zephyrus::prelude::Framework;

use crate::database::IWSCollections;

mod interaction_create;
use interaction_create::*;
mod ready;
use ready::*;
mod ban_add;
use ban_add::*;
mod member_add;
use member_add::*;

pub async fn process_event(
    event: Event,
    http_client: Arc<Client>,
    collections: Arc<IWSCollections>,
    framework: Arc<Framework<Arc<IWSCollections>>>,
) -> eyre::Result<()> {
    match event {
        Event::InteractionCreate(interaction_create_event) => {
            {
                let interaction = interaction_create_event.0.clone();
                let framework = framework.clone();
                tokio::spawn(async move {
                    framework.clone().process(interaction).await;
                });
            }
            let framework = framework.clone();
            let interaction = interaction_create_event.0;
            let http_client = http_client.clone();
            interaction_create(interaction, &http_client, &framework.interaction_client()).await?;
        }
        Event::BanAdd(ban_add_event) => {
            ban_add(ban_add_event, &http_client, &collections).await?;
        }
        Event::Ready(ready_event) => ready(ready_event, &http_client, &collections).await,
        Event::MemberAdd(member_add_event) => {
            member_add(member_add_event, &http_client, &collections).await?;
        }
        _ => (),
    }

    Ok(())
}
