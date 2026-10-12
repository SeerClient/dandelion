//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.KeyMappingAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.KeyMapping)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getKey -> net.minecraft.client.KeyMapping.key
// TODO(fabric2seer): `InputConstants.Key` - return `&obj.key` with its Rust type
// pub fn get_key(obj: &mcrust::net::minecraft::client::key_mapping::KeyMapping) -> &T { &obj.key }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
