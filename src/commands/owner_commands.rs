use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

mod invite;
pub use invite::*;
mod unverify;
pub use unverify::*;
mod announce;
pub use announce::*;
mod remove_report;
pub use remove_report::*;

use crate::BotState;

pub trait OwnerCommands {
    fn owner_commands(self) -> Self;
}

impl OwnerCommands for FrameworkBuilder<Arc<BotState>> {
    fn owner_commands(self) -> Self {
        self.group(|g| {
            g.name("owner")
                .description("Bot Owner commands")
                .command(invite)
                .command(unverify)
                .command(announce)
                .command(remove_report)
        })
    }
}
