//! Ported from net.azureaaron.dandelion.api.patching.ConfigPatch by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;

// TODO(fabric2seer): `.codec(..)` on a E
//   java: Codec<List<ConfigPatch>> PATCH_LIST_CODEC = ConfigPatcher.PATCH_ID_MAPPER.codec(Identifier.CODEC).dispatch(ConfigPatch::codec, Function.identity()).listOf();
/// java: Codec<List<ConfigPatch>> PATCH_LIST_CODEC = ConfigPatcher.PATCH_ID_MAPPER.codec(Identifier.CODEC).dispatch(ConfigPatch::codec, Function.identity()).listOf();
pub static PATCH_LIST_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<dyn ConfigPatch>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PATCH_LIST_CODEC` - see the TODO above"));
/// java: Identifier ID = Dandelion.id("patch/boolean");
pub static OF_BOOLEAN_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("patch/boolean"));
/// java: Identifier ID = Dandelion.id("patch/enum");
pub static OF_ENUM_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("patch/enum"));

/// java: public sealed interface ConfigPatch
pub trait ConfigPatch: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: MapCodec<? extends ConfigPatch> codec(); (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn ConfigPatch>) -> MapCodecRef<compat::Shared<dyn ConfigPatch>>;

    /// java: Optional<Identifier> optionId(); (dispatch: the body for this class)
    fn option_id(&self) -> fn(&compat::Shared<dyn ConfigPatch>) -> Option<Identifier>;

    /// java: String path(); (dispatch: the body for this class)
    fn path(&self) -> fn(&compat::Shared<dyn ConfigPatch>) -> String;

    /// java: List<PatchPredicate> predicates(); (dispatch: the body for this class)
    fn predicates(&self) -> fn(&compat::Shared<dyn ConfigPatch>) -> Vec<compat::Shared<dyn PatchPredicate>>;

}

/// java: non-sealed interface OfBoolean extends ConfigPatch
pub trait OfBoolean: ConfigPatch {
    /// java: boolean value(); (dispatch: the body for this class)
    fn value(&self) -> fn(&compat::Shared<dyn OfBoolean>) -> bool;

}

/// java: non-sealed interface OfEnum extends ConfigPatch
pub trait OfEnum: ConfigPatch {
    /// java: String value(); (dispatch: the body for this class)
    fn value(&self) -> fn(&compat::Shared<dyn OfEnum>) -> String;

}
