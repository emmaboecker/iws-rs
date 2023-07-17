use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

use crate::database::IWSCollections;

mod report;
pub use report::*;
mod report_info;
pub use report_info::*;

pub trait IWSCommands {
    fn iws_commands(self) -> Self;
}

impl IWSCommands for FrameworkBuilder<Arc<IWSCollections>> {
    fn iws_commands(self) -> Self {
        self.command(report)
    }
}
