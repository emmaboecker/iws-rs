use twilight_http::{client::InteractionClient, request::AuditLogReason, Client};
use twilight_model::{channel::message::MessageFlags, guild::Permissions};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::twilight_exports::{
    Interaction, InteractionData, InteractionResponse, InteractionResponseType, InteractionType,
};

pub async fn interaction_create(
    interaction: Interaction,
    http_client: &Client,
    interaction_client: &InteractionClient<'_>,
) -> eyre::Result<()> {
    if interaction.kind != InteractionType::MessageComponent {
        return Ok(());
    }

    let data = interaction.data.unwrap();

    if let InteractionData::MessageComponent(data) = data {
        let mut split = data.custom_id.split(':');
        if split.next() != Some("ban") {
            return Ok(());
        }

        let member = interaction.member;

        if member.is_none() {
            return Err(eyre::eyre!("Could not get member from interaction"));
        }

        let member = member.unwrap();

        let permissions = member.permissions.unwrap_or_else(Permissions::empty);

        let mut response_data =
            InteractionResponseDataBuilder::new().flags(MessageFlags::EPHEMERAL);

        response_data = if !permissions.contains(Permissions::BAN_MEMBERS) {
            response_data.content("Du kannst keine Mitglieder bannen!")
        } else {
            let user_id = split.next().unwrap();

            let create_ban = http_client
                .create_ban(interaction.guild_id.unwrap(), user_id.parse().unwrap())
                .reason(&format!(
                    "Gebannt durch Button von {}",
                    member.user.unwrap().name
                ))
                .unwrap()
                .await;

            if let Err(e) = create_ban {
                response_data.content(format!("User konnte nicht gebannt werden: `{}`", e))
            } else {
                response_data.content(format!("<@!{}> wurde gebannt!", user_id))
            }
        };

        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(response_data.build()),
                },
            )
            .await?;
    }

    Ok(())
}
