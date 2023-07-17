use serde::{Deserialize, Serialize};
use twilight_model::id::Id;
use zephyrus::twilight_exports::{ChannelMarker, GuildMarker, RoleMarker};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotSettings {
    #[serde(rename = "_id")]
    pub guild_id: Id<GuildMarker>,
    #[serde(default)]
    pub log_channel: Option<Id<ChannelMarker>>,
    #[serde(default)]
    pub ping_roles: Vec<Id<RoleMarker>>,
    #[serde(default = "bool_true")]
    pub auto_report: bool,
}

fn bool_true() -> bool {
    true
}
