//! Ported from net.azureaaron.dandelion.impl.patching.EnumPatch by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::api::patching::config_patch::OfEnum;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: protected static final MapCodec<EnumPatch> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group( Identifier.CODEC.optionalFieldOf("optionId").forGetter(EnumPatch::optionId), Codec.STRING
pub static MAP_CODEC: compat::MainThread<MapCodecRef<EnumPatch>> = compat::MainThread::new(|| record_codec_builder::map_codec(record_codec_builder::apply4(record_codec_builder::of(|o: &EnumPatch| o.option_id(), codec::optional_field_of_opt(mcrust::net::minecraft::resources::identifier::Identifier::codec(), "optionId")), record_codec_builder::of(|o: &EnumPatch| o.path(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "path")), record_codec_builder::of(|o: &EnumPatch| o.value(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "value")), record_codec_builder::of(|o: &EnumPatch| o.predicates(), codec::optional_field_of(config_patcher::PATCH_PREDICATES_CODEC.get_mut().clone(), "predicates", vec![])), move |__a0, __a1, __a2, __a3| EnumPatch::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));

/// java: public record EnumPatch(Optional<Identifier> optionId, String path, String value, List<PatchPredicate> predicates) implements ConfigPatch.OfEnum
#[derive(Clone, Debug)]
pub struct EnumPatch {
    /// java: Optional<Identifier> optionId
    pub option_id: Option<Identifier>,
    /// java: String path
    pub path: String,
    /// java: String value
    pub value: String,
    /// java: List<PatchPredicate> predicates
    pub predicates: Vec<compat::Shared<dyn PatchPredicate>>,
}

impl EnumPatch {
    /// java: the canonical record constructor
    pub fn new(mut option_id: Option<Identifier>, mut path: String, mut value: String, mut predicates: Vec<compat::Shared<dyn PatchPredicate>>) -> Self {
        Self { option_id, path, value, predicates }
    }

    /// java: the record accessor `optionId()`
    pub fn option_id(&self) -> Option<Identifier> {
        self.option_id.clone()
    }

    /// java: the record accessor `path()`
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// java: the record accessor `value()`
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// java: the record accessor `predicates()`
    pub fn predicates(&self) -> Vec<compat::Shared<dyn PatchPredicate>> {
        self.predicates.clone()
    }

}

impl OfEnum for EnumPatch {
}

impl ConfigPatch for EnumPatch {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public MapCodec<? extends ConfigPatch> codec() (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn ConfigPatch>) -> MapCodecRef<compat::Shared<dyn ConfigPatch>> {
        EnumPatch::codec_dyn
    }

}

impl EnumPatch {
    /// java: @Override public MapCodec<? extends ConfigPatch> codec()
    pub fn codec_impl(&self) -> MapCodecRef<compat::Shared<dyn ConfigPatch>> {
        return MAP_CODEC.get_mut().clone();
    }

    /// EnumPatch.codec as the dispatch of ConfigPatch.codec
    pub fn codec_dyn(__this: &compat::Shared<dyn ConfigPatch>) -> MapCodecRef<compat::Shared<dyn ConfigPatch>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.codec_impl())
    }

}
