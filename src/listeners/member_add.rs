use mongodb::bson::doc;
use twilight_http::Client;
use twilight_model::{channel::message::component::Button, gateway::payload::incoming::MemberAdd};
use zephyrus::twilight_exports::{ActionRow, Component};

use crate::{database::IWSCollections, utils::report_embed};

pub async fn member_add(
    member_add: Box<MemberAdd>,
    http_client: &Client,
    collections: &IWSCollections,
) -> eyre::Result<()> {
    let settings = collections
        .bot_settings
        .find_one(doc! { "_id": member_add.guild_id.to_string() }, None)
        .await?;

    if settings.is_none() {
        return Ok(());
    }

    let settings = settings.unwrap();

    if settings.log_channel.is_none() {
        return Ok(());
    }

    let report_info = collections
        .reported_users
        .find_one(doc! { "_id": member_add.user.clone().id.to_string() }, None)
        .await?;

    if report_info.is_none() {
        return Ok(());
    }

    let report_info = report_info.unwrap();

    let embed_builder = report_embed(
        "Gemeldeter User ist gerade beigetreten!",
        "Es ist ein auf einem anderen Server gemeldeten User diesem Server beigetreten!",
        &report_info,
        http_client,
        Some(member_add.user.clone()),
    )
    .await?;

    let embed = embed_builder.build();

    http_client
        .create_message(settings.log_channel.unwrap())
        .content(
            &settings
                .ping_roles
                .into_iter()
                .map(|role| format!("<@&{}>", role))
                .collect::<Vec<_>>()
                .join(", "),
        )?
        .embeds(&[embed])?
        .components(&[Component::ActionRow(ActionRow {
            components: vec![Component::Button(Button {
                custom_id: Some(format!("ban:{}", member_add.user.id)),
                disabled: false,
                emoji: None,
                style: twilight_model::channel::message::component::ButtonStyle::Danger,
                label: Some("User Bannen".to_string()),
                url: None,
            })],
        })])?
        .await?;

    Ok(())
}
