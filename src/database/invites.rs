use serde::{Deserialize, Serialize};
use twilight_model::id::Id;
use zephyrus::twilight_exports::GuildMarker;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    #[serde(rename = "_id")]
    pub invite: String,
    pub guild_id: Id<GuildMarker>,
}
