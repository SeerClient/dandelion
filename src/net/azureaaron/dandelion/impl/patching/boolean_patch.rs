//! Ported from net.azureaaron.dandelion.impl.patching.BooleanPatch.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::{ConfigPatch, OfBoolean, PatchCodec};
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: protected static final MapCodec<BooleanPatch> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group(
/// Identifier.CODEC.optionalFieldOf("optionId").forGetter(BooleanPatch::optionId), Codec.STRING.fieldOf("path").forGetter(..),
/// Codec.BOOL.fieldOf("value").forGetter(..), ConfigPatcher.PATCH_PREDICATES_CODEC.optionalFieldOf("predicates", List.of())
/// .forGetter(..)).apply(instance, BooleanPatch::new));
///
/// Held upcast to `MapCodec<? extends ConfigPatch>` (`map_codec::upcast`): the same object `codec()` answers and the
/// id mapper holds.
pub static MAP_CODEC: compat::MainThread<PatchCodec> = compat::MainThread::new(|| {
    let record = record_codec_builder::map_codec(record_codec_builder::apply4(
        record_codec_builder::of(|o: &BooleanPatch| o.option_id.clone(), codec::optional_field_of_opt(Identifier::codec(), "optionId")),
        record_codec_builder::of(|o: &BooleanPatch| o.path.clone(), codec::field_of(codec::string_codec(), "path")),
        record_codec_builder::of(|o: &BooleanPatch| o.value.clone(), codec::field_of(codec::bool_codec(), "value")),
        record_codec_builder::of(|o: &BooleanPatch| o.predicates.clone(), codec::optional_field_of(config_patcher::PATCH_PREDICATES_CODEC.get().clone(), "predicates", Vec::new())),
        |option_id, path, value, predicates| BooleanPatch::new(option_id, path, value, predicates),
    ));
    PatchCodec(map_codec::upcast(
        record,
        |p: BooleanPatch| Arc::new(p) as Arc<dyn ConfigPatch>,
        |p: &dyn ConfigPatch| p.dyn_any().downcast_ref::<BooleanPatch>(),
    ))
});

/// java: public record BooleanPatch(Optional<Identifier> optionId, String path, boolean value, List<PatchPredicate> predicates)
/// implements ConfigPatch.OfBoolean
#[derive(Clone)]
pub struct BooleanPatch {
    /// java: Optional<Identifier> optionId
    pub option_id: Option<Identifier>,
    /// java: String path
    pub path: String,
    /// java: boolean value
    pub value: bool,
    /// java: List<PatchPredicate> predicates
    pub predicates: Vec<Arc<dyn PatchPredicate>>,
}

impl BooleanPatch {
    /// java: the canonical record constructor
    pub fn new(option_id: Option<Identifier>, path: String, value: bool, predicates: Vec<Arc<dyn PatchPredicate>>) -> Self {
        Self { option_id, path, value, predicates }
    }
}

impl ConfigPatch for BooleanPatch {
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

    fn as_of_boolean(&self) -> Option<&dyn OfBoolean> {
        Some(self)
    }
}

impl OfBoolean for BooleanPatch {
    /// java: the record accessor `value()`
    fn value(&self) -> bool {
        self.value.clone()
    }
}
