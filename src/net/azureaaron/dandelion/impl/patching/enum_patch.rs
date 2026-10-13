//! Ported from net.azureaaron.dandelion.impl.patching.EnumPatch.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::{ConfigPatch, OfEnum, PatchCodec};
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: protected static final MapCodec<EnumPatch> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group(
/// Identifier.CODEC.optionalFieldOf("optionId").forGetter(EnumPatch::optionId), Codec.STRING.fieldOf("path").forGetter(..),
/// Codec.STRING.fieldOf("value").forGetter(..), ConfigPatcher.PATCH_PREDICATES_CODEC.optionalFieldOf("predicates", List.of())
/// .forGetter(..)).apply(instance, EnumPatch::new));
///
/// Held upcast to `MapCodec<? extends ConfigPatch>` (`map_codec::upcast`): the same object `codec()` answers and the
/// id mapper holds.
pub static MAP_CODEC: compat::MainThread<PatchCodec> = compat::MainThread::new(|| {
    let record = record_codec_builder::map_codec(record_codec_builder::apply4(
        record_codec_builder::of(|o: &EnumPatch| o.option_id.clone(), codec::optional_field_of_opt(Identifier::codec(), "optionId")),
        record_codec_builder::of(|o: &EnumPatch| o.path.clone(), codec::field_of(codec::string_codec(), "path")),
        record_codec_builder::of(|o: &EnumPatch| o.value.clone(), codec::field_of(codec::string_codec(), "value")),
        record_codec_builder::of(|o: &EnumPatch| o.predicates.clone(), codec::optional_field_of(config_patcher::PATCH_PREDICATES_CODEC.get().clone(), "predicates", Vec::new())),
        |option_id, path, value, predicates| EnumPatch::new(option_id, path, value, predicates),
    ));
    PatchCodec(map_codec::upcast(
        record,
        |p: EnumPatch| Arc::new(p) as Arc<dyn ConfigPatch>,
        |p: &dyn ConfigPatch| p.dyn_any().downcast_ref::<EnumPatch>(),
    ))
});

/// java: public record EnumPatch(Optional<Identifier> optionId, String path, String value, List<PatchPredicate> predicates)
/// implements ConfigPatch.OfEnum
#[derive(Clone)]
pub struct EnumPatch {
    /// java: Optional<Identifier> optionId
    pub option_id: Option<Identifier>,
    /// java: String path
    pub path: String,
    /// java: String value
    pub value: String,
    /// java: List<PatchPredicate> predicates
    pub predicates: Vec<Arc<dyn PatchPredicate>>,
}

impl EnumPatch {
    /// java: the canonical record constructor
    pub fn new(option_id: Option<Identifier>, path: String, value: String, predicates: Vec<Arc<dyn PatchPredicate>>) -> Self {
        Self { option_id, path, value, predicates }
    }
}

impl ConfigPatch for EnumPatch {
    /// java: @Override public MapCodec<? extends ConfigPatch> codec()
    fn codec(&self) -> PatchCodec {
        MAP_CODEC.get().clone()
    }

    /// java: the record accessor `optionId()`
    fn option_id(&self) -> Option<Identifier> {
        self.option_id.clone()
    }

    /// java: the record accessor `path()`
    fn path(&self) -> String {
        self.path.clone()
    }

    /// java: the record accessor `predicates()`
    fn predicates(&self) -> Vec<Arc<dyn PatchPredicate>> {
        self.predicates.clone()
    }

    fn as_of_enum(&self) -> Option<&dyn OfEnum> {
        Some(self)
    }
}

impl OfEnum for EnumPatch {
    /// java: the record accessor `value()`
    fn value(&self) -> String {
        self.value.clone()
    }
}
