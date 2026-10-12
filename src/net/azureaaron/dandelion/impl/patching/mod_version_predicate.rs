//! Ported from net.azureaaron.dandelion.impl.patching.ModVersionPredicate by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::ComparisonOperator;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::OfModVersion;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::utils::codec_utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;

/// java: protected static final MapCodec<ModVersionPredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group( Codec.STRING.fieldOf("modId").forGetter(ModVersionPredicate::modId), CodecUtils
pub static MAP_CODEC: compat::MainThread<MapCodecRef<ModVersionPredicate>> = compat::MainThread::new(|| record_codec_builder::map_codec(record_codec_builder::apply3(record_codec_builder::of(|o: &ModVersionPredicate| o.mod_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "modId")), record_codec_builder::of(|o: &ModVersionPredicate| o.version(), codec::field_of(codec_utils::SEMANTIC_VERSION_CODEC.get_mut().clone(), "version")), record_codec_builder::of(|o: &ModVersionPredicate| o.operator(), codec::field_of(comparison_operator::CODEC.get_mut().clone(), "operator")), move |__a0, __a1, __a2| ModVersionPredicate::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public record ModVersionPredicate(String modId, SemanticVersion version, ComparisonOperator operator) implements PatchPredicate.OfModVersion
pub struct ModVersionPredicate {
    /// java: String modId
    pub mod_id: String,
    /// java: SemanticVersion version
    pub version: mcrust::mcrust::modding::version::SemanticVersion,
    /// java: ComparisonOperator operator
    pub operator: ComparisonOperator,
}

impl ModVersionPredicate {
    /// java: the canonical record constructor
    pub fn new(mut mod_id: String, mut version: mcrust::mcrust::modding::version::SemanticVersion, mut operator: ComparisonOperator) -> Self {
        Self { mod_id, version, operator }
    }

    /// java: the record accessor `modId()`
    pub fn mod_id(&self) -> String {
        self.mod_id.clone()
    }

    /// java: the record accessor `version()`
    pub fn version(&self) -> mcrust::mcrust::modding::version::SemanticVersion {
        self.version.clone()
    }

    /// java: the record accessor `operator()`
    pub fn operator(&self) -> ComparisonOperator {
        self.operator
    }

}

impl OfModVersion for ModVersionPredicate {
}

impl PatchPredicate for ModVersionPredicate {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public MapCodec<? extends PatchPredicate> codec() (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        ModVersionPredicate::codec_dyn
    }

    /// java: @Override public boolean test() (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> bool {
        ModVersionPredicate::test_dyn
    }

}

impl ModVersionPredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    pub fn codec_impl(&self) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        return MAP_CODEC.get_mut().clone();
    }

    /// ModVersionPredicate.codec as the dispatch of PatchPredicate.codec
    pub fn codec_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.codec_impl())
    }

    /// java: @Override public boolean test()
    pub fn test_impl(&self) -> bool {
        let mut mod_container: Option<std::sync::Arc<mcrust::mcrust::modding::loader::ModContainer>> = mcrust::mcrust::modding::loader::get_mod(&self.mod_id());
        if mod_container.is_some() {
            let current_version = &mod_container.clone().unwrap().metadata.version;
            return self.operator().compare(current_version, &self.version());
        }
        return false;
    }

    /// ModVersionPredicate.test as the dispatch of PatchPredicate.test
    pub fn test_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> bool {
        compat::read_as::<Self, _, _>(__this, |__s| __s.test_impl())
    }

}
