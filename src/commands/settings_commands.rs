use std::sync::Arc;

use zephyrus::prelude::FrameworkBuilder;

use crate::database::IWSCollections;

mod warning_channel;
use warning_channel::*;
mod ping_roles;
use ping_roles::*;
mod auto_report;
use auto_report::*;

pub trait SettingsCommands {
    fn settings_commands(self) -> Self;
}

impl SettingsCommands for FrameworkBuilder<Arc<IWSCollections>> {
    fn settings_commands(self) -> Self {
        self.group(|g| {
            g.name("settings")
                .description("Bot Einstellungen")
                .command(warning_channel)
                .command(ping_roles)
                .command(auto_report)
        })
    }
}
