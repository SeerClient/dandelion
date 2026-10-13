//! Ported from net.azureaaron.dandelion.api.patching.PatchPredicateRegistry.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PredicateCodec;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public final class PatchPredicateRegistry
///
/// Allows users of Dandelion to register their own custom `PatchPredicate`s.
pub struct PatchPredicateRegistry {}

impl PatchPredicateRegistry {
    /// java: public static void register(Identifier id, MapCodec<? extends PatchPredicate> codec)
    ///
    /// `codec` is the predicate record's codec upcast to the interface (`map_codec::upcast`); the predicate's own
    /// `codec()` must answer that same `PredicateCodec`.
    pub fn register(id: Identifier, codec: PredicateCodec) {
        config_patcher::PREDICATE_ID_MAPPER.get_mut().put(id, codec);
    }
}
