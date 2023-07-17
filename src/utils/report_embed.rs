use twilight_http::Client;
use twilight_model::user::User;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::database::ReportedUser;

pub async fn report_embed(
    title: &str,
    description: &str,
    report_info: &ReportedUser,
    http_client: &Client,
    user: Option<User>,
) -> eyre::Result<EmbedBuilder> {
    let guild = http_client
        .guild(report_info.reported_on_server)
        .await?
        .model()
        .await?;

    let reported_by = if let Some(reported_by) = report_info.reported_by {
        Some(http_client.user(reported_by.cast()).await?.model().await?)
    } else {
        None
    };

    let user = if let Some(user) = user {
        user
    } else {
        http_client
            .user(report_info.discord_id)
            .await?
            .model()
            .await?
    };

    let mut embed_builder = EmbedBuilder::new()
        .title(title)
        .description(description)
        .field(EmbedFieldBuilder::new(
            "User",
            format!("<@{}> ({} / {})", user.id, user.name, user.id),
        ))
        .field(EmbedFieldBuilder::new(
            "Gründe",
            report_info
                .reasons
                .iter()
                .map(|reason| format!("- {}", reason))
                .collect::<Vec<_>>()
                .join("\n"),
        ))
        .field(EmbedFieldBuilder::new(
            "Gemeldet auf",
            format!("{} ({})", guild.name, guild.id),
        ));

    if let Some(reported_by) = reported_by {
        embed_builder = embed_builder.field(EmbedFieldBuilder::new(
            "Gemeldet von",
            format!(
                "<@{}> ({} / {})",
                reported_by.id, reported_by.name, reported_by.id
            ),
        ));
    }

    if let Some(reported_at) = report_info.reported_at {
        embed_builder = embed_builder.field(EmbedFieldBuilder::new(
            "Gemeldet",
            format!("<t:{}:f>", reported_at.timestamp()),
        ));
    }

    Ok(embed_builder)
}
