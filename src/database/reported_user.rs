use serde::{Deserialize, Serialize};
use twilight_model::{id::Id, user::User};
use zephyrus::twilight_exports::{GuildMarker, UserMarker};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportedUser {
    #[serde(rename = "_id")]
    pub discord_id: Id<UserMarker>,
    pub reported_on_server: Id<GuildMarker>,
    pub reported_by: Option<Id<User>>,
    pub reported_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reasons: Vec<String>,
}
