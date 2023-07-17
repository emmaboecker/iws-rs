use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

mod invite;
mod unverify;
pub use invite::*;
pub use unverify::*;

use crate::database::IWSCollections;

pub trait VerificationCommands {
    fn verification_commands(self) -> Self;
}

impl VerificationCommands for FrameworkBuilder<Arc<IWSCollections>> {
    fn verification_commands(self) -> Self {
        self.command(invite).command(unverify)
    }
}
