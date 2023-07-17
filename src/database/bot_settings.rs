use serde::{Deserialize, Serialize};
use twilight_model::{channel::Channel, guild::Role, id::Id};
use zephyrus::twilight_exports::GuildMarker;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotSettings {
    #[serde(rename = "_id")]
    pub guild_id: Id<GuildMarker>,
    #[serde(default)]
    pub log_channel: Option<Id<Channel>>,
    #[serde(default)]
    pub ping_roles: Vec<Id<Role>>,
    #[serde(default = "bool_true")]
    pub auto_report: bool,
}

fn bool_true() -> bool {
    true
}
