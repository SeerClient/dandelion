//! Ported from net.azureaaron.dandelion.api.patching.PatchPredicateRegistry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public final class PatchPredicateRegistry
#[derive(Clone)]
pub struct PatchPredicateRegistry {}

impl PatchPredicateRegistry {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void register(Identifier id, MapCodec<? extends PatchPredicate> codec)
    pub fn register(id: &Identifier, codec: &MapCodecRef<compat::Shared<dyn PatchPredicate>>) {
        config_patcher::PREDICATE_ID_MAPPER.get_mut().put(id.clone(), codec.clone());
    }

}
