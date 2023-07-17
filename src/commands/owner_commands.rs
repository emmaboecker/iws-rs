use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

mod invite;
pub use invite::*;
mod unverify;
pub use unverify::*;
mod announce;
pub use announce::*;

use crate::database::IWSCollections;

pub trait OwnerCommands {
    fn owner_commands(self) -> Self;
}

impl OwnerCommands for FrameworkBuilder<Arc<IWSCollections>> {
    fn owner_commands(self) -> Self {
        self.group(|g| {
            g.name("owner")
                .description("Bot Owner commands")
                .command(invite)
                .command(unverify)
                .command(announce)
        })
    }
}
