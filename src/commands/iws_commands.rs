use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

use crate::BotState;

mod report;
pub use report::*;
mod report_info;
pub use report_info::*;
mod scan;
pub use scan::*;

pub trait IWSCommands {
    fn iws_commands(self) -> Self;
}

impl IWSCommands for FrameworkBuilder<Arc<BotState>> {
    fn iws_commands(self) -> Self {
        self.command(report).command(report_info).command(scan)
    }
}
