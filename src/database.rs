use mongodb::Collection;

mod bot_settings;
pub use bot_settings::*;
mod reported_user;
pub use reported_user::*;
mod scan_cooldown;
pub use scan_cooldown::*;
mod invites;
pub use invites::*;
mod verified_guild;
pub use verified_guild::*;

#[derive(Clone)]
pub struct IWSCollections {
    pub reported_users: Collection<ReportedUser>,
    pub bot_settings: Collection<BotSettings>,
    pub scan_cooldown: Collection<ScanCooldown>,

    pub invites: Collection<Invitation>,
    pub verified_guilds: Collection<VerifiedGuild>,
}
