//! Ported from net.azureaaron.dandelion.impl.patching.ModVersionPredicate.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::{self, ComparisonOperator};
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::{OfModVersion, PatchPredicate, PredicateCodec};
use crate::net::azureaaron::dandelion::r#impl::utils::codec_utils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::mcrust::modding::loader;
use mcrust::mcrust::modding::version::{SemanticVersion, Version};

/// java: protected static final MapCodec<ModVersionPredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance ->
/// instance.group(Codec.STRING.fieldOf("modId").forGetter(ModVersionPredicate::modId),
/// CodecUtils.SEMANTIC_VERSION_CODEC.fieldOf("version").forGetter(ModVersionPredicate::version),
/// ComparisonOperator.CODEC.fieldOf("operator").forGetter(ModVersionPredicate::operator))
/// .apply(instance, ModVersionPredicate::new));
///
/// Held upcast to `MapCodec<? extends PatchPredicate>` (`map_codec::upcast`): the same object `codec()` answers and
/// the id mapper holds.
pub static MAP_CODEC: compat::MainThread<PredicateCodec> = compat::MainThread::new(|| {
    let record = record_codec_builder::map_codec(record_codec_builder::apply3(
        record_codec_builder::of(|o: &ModVersionPredicate| o.mod_id.clone(), codec::field_of(codec::string_codec(), "modId")),
        record_codec_builder::of(|o: &ModVersionPredicate| o.version.clone(), codec::field_of(codec_utils::SEMANTIC_VERSION_CODEC.clone(), "version")),
        record_codec_builder::of(|o: &ModVersionPredicate| o.operator, codec::field_of(comparison_operator::CODEC.clone(), "operator")),
        |mod_id, version, operator| ModVersionPredicate::new(mod_id, version, operator),
    ));
    PredicateCodec(map_codec::upcast(
        record,
        |p: ModVersionPredicate| Arc::new(p) as Arc<dyn PatchPredicate>,
        |p: &dyn PatchPredicate| p.dyn_any().downcast_ref::<ModVersionPredicate>(),
    ))
});

/// java: public record ModVersionPredicate(String modId, SemanticVersion version, ComparisonOperator operator)
/// implements PatchPredicate.OfModVersion
#[derive(Clone, Debug)]
pub struct ModVersionPredicate {
    /// java: String modId
    pub mod_id: String,
    /// java: SemanticVersion version
    pub version: SemanticVersion,
    /// java: ComparisonOperator operator
    pub operator: ComparisonOperator,
}

impl ModVersionPredicate {
    /// java: the canonical record constructor
    pub fn new(mod_id: String, version: SemanticVersion, operator: ComparisonOperator) -> Self {
        Self { mod_id, version, operator }
    }
}

impl PatchPredicate for ModVersionPredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    fn codec(&self) -> PredicateCodec {
        MAP_CODEC.get().clone()
    }

    /// java: @Override public boolean test()
    fn test(&self) -> bool {
        // java: Optional<ModContainer> modContainer = FabricLoader.getInstance().getModContainer(this.modId());
        if let Some(mod_container) = loader::get_mod(&self.mod_id) {
            let current_version: &Version = &mod_container.metadata.version;
            return self.operator.compare(current_version, &Version::Semantic(self.version.clone()));
        }
        false
    }
}

impl OfModVersion for ModVersionPredicate {
    /// java: the record accessor `modId()`
    fn mod_id(&self) -> String {
        self.mod_id.clone()
    }

    /// java: the record accessor `version()`
    fn version(&self) -> SemanticVersion {
        self.version.clone()
    }

    /// java: the record accessor `operator()`
    fn operator(&self) -> ComparisonOperator {
        self.operator
    }
}
