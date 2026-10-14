//! Dandelion - ported from Fabric by fabric2seer (SeerClient native mod).
//! See PORTING_REPORT.md for what was translated and what is left as TODO.
#![allow(unused_imports, unused_mut, unused_variables, unused_parens, dead_code, non_snake_case,
         clippy::all)]

// The game crate. Linked as the prebuilt SDK `mcrust.dll` (`-L crate=<sdk>/bin`);
// an explicit `extern crate` because a command-line `--extern mcrust` breaks
// resolving it through another mod (dandelion -> yacl).
extern crate mcrust;

pub mod compat;
pub mod prelude;
pub mod mixins;
pub mod entry;
pub mod net;
pub mod unresolved;

use mcrust::mcrust::modding::entrypoint::{ClientModInitializer, ModInitializer, PreLaunchEntrypoint};

/// Registers every ported Mixin (preLaunch: before the game bootstraps, as Mixin applies).
#[derive(Default)]
pub struct Mixins;

impl PreLaunchEntrypoint for Mixins {
    fn on_pre_launch(&self) {
        mixins::gui_option_editor_dropdown_accessor::register();
        mixins::key_mapping_accessor::register();
        mixins::moul_config_screen_component_mixin::register();
    }
}

/// the "client" entrypoint
#[derive(Default)]
pub struct Client;

impl ClientModInitializer for Client {
    fn on_initialize_client(&self) {
        compat::install(); // Minecraft.getInstance() for the translated classes
        entry::dandelion::on_initialize_client();
    }
}

mcrust::declare_mod! {
    client: [Client],
    pre_launch: [Mixins],
}
