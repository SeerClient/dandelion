//! Ported from net.azureaaron.dandelion.api.patching.ConfigPatch.
//!
//! A patch that can be applied to a value in a config class.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: Codec<List<ConfigPatch>> PATCH_LIST_CODEC = ConfigPatcher.PATCH_ID_MAPPER.codec(Identifier.CODEC)
/// .dispatch(ConfigPatch::codec, Function.identity()).listOf();
///
/// The `Codec` used to deserialize config patches. The usage of this `Codec` off-thread is not recommended.
pub static PATCH_LIST_CODEC: compat::MainThread<CodecRef<Vec<Arc<dyn ConfigPatch>>>> = compat::MainThread::new(|| {
    let key_codec = config_patcher::PATCH_ID_MAPPER.get().codec(Identifier::codec());
    codec::list(codec::dispatch(key_codec, |patch: &Arc<dyn ConfigPatch>| patch.codec(), |c: &PatchCodec| c.0.clone()))
});

/// java: Identifier ID = Dandelion.id("patch/boolean");
#[allow(non_snake_case)]
pub fn OF_BOOLEAN_ID() -> Identifier {
    crate::entry::dandelion::id("patch/boolean")
}

/// java: Identifier ID = Dandelion.id("patch/enum");
#[allow(non_snake_case)]
pub fn OF_ENUM_ID() -> Identifier {
    crate::entry::dandelion::id("patch/enum")
}

/// java: `MapCodec<? extends ConfigPatch>` - a patch record's codec seen as the codec of the interface
/// (`map_codec::upcast`), compared by identity (the `LateBoundIdMapper`'s `BiMap` key).
#[derive(Clone)]
pub struct PatchCodec(pub MapCodecRef<Arc<dyn ConfigPatch>>);

impl PartialEq for PatchCodec {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(Arc::as_ptr(&self.0) as *const (), Arc::as_ptr(&other.0) as *const ())
    }
}

/// java: public sealed interface ConfigPatch
pub trait ConfigPatch: compat::DynAny + Send + Sync {
    /// java: MapCodec<? extends ConfigPatch> codec();
    fn codec(&self) -> PatchCodec;

    /// java: Optional<Identifier> optionId();
    fn option_id(&self) -> Option<Identifier>;

    /// java: String path();
    fn path(&self) -> String;

    /// java: List<PatchPredicate> predicates();
    fn predicates(&self) -> Vec<Arc<dyn PatchPredicate>>;

    /// `patch instanceof ConfigPatch.OfBoolean booleanPatch`
    fn as_of_boolean(&self) -> Option<&dyn OfBoolean> {
        None
    }

    /// `patch instanceof ConfigPatch.OfEnum enumPatch`
    fn as_of_enum(&self) -> Option<&dyn OfEnum> {
        None
    }
}

/// java: non-sealed interface OfBoolean extends ConfigPatch - config patch for overriding boolean values.
pub trait OfBoolean: ConfigPatch {
    /// java: boolean value();
    fn value(&self) -> bool;
}

/// java: non-sealed interface OfEnum extends ConfigPatch - config patch for overriding Enum-based values.
pub trait OfEnum: ConfigPatch {
    /// java: String value();
    fn value(&self) -> String;
}
