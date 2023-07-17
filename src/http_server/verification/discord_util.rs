use reqwest::Url;
use twilight_model::id::Id;
use zephyrus::twilight_exports::GuildMarker;

pub fn generate_discord_path(guild_id: Id<GuildMarker>, state: String) -> String {
    let client_id = std::env::var("DISCORD_CLIENT_ID").unwrap();

    let redirect_url = Url::parse(&format!("{}/done", std::env::var("WEBSERVER_URL").unwrap()))
        .unwrap()
        .to_string();

    let url = Url::parse("https://discord.com/api/oauth2/authorize")
        .unwrap()
        .query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("permissions", "8")
        .append_pair("scope", "bot applications.commands identify")
        .append_pair("guild_id", &guild_id.to_string())
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &redirect_url)
        .append_pair("state", &state)
        .append_pair("disable_guild_select", "true")
        .finish()
        .to_string();

    url
}
