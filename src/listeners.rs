mod interaction_create;
use std::sync::Arc;

mod ready;
use twilight_gateway::Event;
use twilight_http::Client;
use zephyrus::prelude::Framework;

use crate::database::IWSCollections;

use interaction_create::*;
use ready::*;

pub async fn process_event(
    event: Event,
    http_client: Arc<Client>,
    collections: Arc<IWSCollections>,
    framework: Arc<Framework<Arc<IWSCollections>>>,
) {
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
            interaction_create(interaction, &http_client, &framework.interaction_client())
                .await
                .unwrap();
        }
        Event::MemberAdd(member_add) => {
            tracing::info!(?member_add, "member added");
        }
        Event::Ready(ready_event) => ready(ready_event, &http_client, &collections).await,
        _ => (),
    }
}
