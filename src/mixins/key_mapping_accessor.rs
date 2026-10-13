//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.KeyMappingAccessor.
//! @Mixin(net.minecraft.client.KeyMapping)

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::net::minecraft::client::key_mapping::{self, KeyMapping};

/// @Accessor getKey -> net.minecraft.client.KeyMapping.key
///
/// The port keeps the LIVE binding of a registered mapping in `KeyMapping.ALL` (`key_mapping::key_of`); a mapping
/// that is not registered answers its own `key` field.
pub fn get_key(obj: &KeyMapping) -> Key {
    key_mapping::key_of(obj.get_name()).unwrap_or_else(|| obj.key.clone())
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (an accessor needs no registration: `key` is a public field of the port's KeyMapping)
}
