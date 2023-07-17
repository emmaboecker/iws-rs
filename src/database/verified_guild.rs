use serde::{Deserialize, Serialize};
use twilight_model::id::Id;
use zephyrus::twilight_exports::GuildMarker;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedGuild {
    #[serde(rename = "_id")]
    pub guild_id: Id<GuildMarker>,
    pub verified: bool,
}