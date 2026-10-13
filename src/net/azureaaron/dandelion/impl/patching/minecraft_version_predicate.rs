//! Ported from net.azureaaron.dandelion.impl.patching.MinecraftVersionPredicate.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::{self, ComparisonOperator};
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::{OfMinecraftVersion, PatchPredicate, PredicateCodec};
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::mcrust::modding::version::{SemanticVersion, Version};
use mcrust::net::minecraft::shared_constants;

/// java: protected static final MapCodec<MinecraftVersionPredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance ->
/// instance.group(Codec.STRING.fieldOf("version").forGetter(MinecraftVersionPredicate::version),
/// ComparisonOperator.CODEC.fieldOf("operator").forGetter(MinecraftVersionPredicate::operator))
/// .apply(instance, MinecraftVersionPredicate::new));
///
/// Held upcast to `MapCodec<? extends PatchPredicate>` (`map_codec::upcast`): the same object `codec()` answers and
/// the id mapper holds.
pub static MAP_CODEC: compat::MainThread<PredicateCodec> = compat::MainThread::new(|| {
    let record = record_codec_builder::map_codec(record_codec_builder::apply2(
        record_codec_builder::of(|o: &MinecraftVersionPredicate| o.version.clone(), codec::field_of(codec::string_codec(), "version")),
        record_codec_builder::of(|o: &MinecraftVersionPredicate| o.operator, codec::field_of(comparison_operator::CODEC.clone(), "operator")),
        |version, operator| MinecraftVersionPredicate::new(version, operator),
    ));
    PredicateCodec(map_codec::upcast(
        record,
        |p: MinecraftVersionPredicate| Arc::new(p) as Arc<dyn PatchPredicate>,
        |p: &dyn PatchPredicate| p.dyn_any().downcast_ref::<MinecraftVersionPredicate>(),
    ))
});

/// java: public record MinecraftVersionPredicate(String version, ComparisonOperator operator) implements
/// PatchPredicate.OfMinecraftVersion
#[derive(Clone, Debug, PartialEq)]
pub struct MinecraftVersionPredicate {
    /// java: String version
    pub version: String,
    /// java: ComparisonOperator operator
    pub operator: ComparisonOperator,
}

impl MinecraftVersionPredicate {
    /// java: the canonical record constructor
    pub fn new(version: String, operator: ComparisonOperator) -> Self {
        Self { version, operator }
    }
}

impl PatchPredicate for MinecraftVersionPredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    fn codec(&self) -> PredicateCodec {
        MAP_CODEC.get().clone()
    }

    /// java: @Override public boolean test()
    fn test(&self) -> bool {
        let current_version_id: String = shared_constants::get_current_version().id.clone();
        // rust: port gap (GAPS_D.md D4): Fabric Loader's `McVersionLookup` (normalizeVersion / getRelease) is not in
        // the port.
        //   java: String normalizedCurrent = McVersionLookup.normalizeVersion(currentVersionId, McVersionLookup.getRelease(currentVersionId));
        //   java: String normalizedTarget = McVersionLookup.normalizeVersion(this.version(), McVersionLookup.getRelease(this.version()));
        let _ = current_version_id;
        let normalized_current: String = todo!("fabric2seer: McVersionLookup.normalizeVersion (GAPS_D.md D4)");
        let normalized_target: String = todo!("fabric2seer: McVersionLookup.normalizeVersion (GAPS_D.md D4)");
        // java: try {..} catch (VersionParsingException _) {}
        if let (Ok(current_version), Ok(target_version)) = (SemanticVersion::parse(&normalized_current), SemanticVersion::parse(&normalized_target)) {
            return self.operator.compare(&Version::Semantic(current_version), &Version::Semantic(target_version));
        }
        false
    }
}

impl OfMinecraftVersion for MinecraftVersionPredicate {
    /// java: the record accessor `version()`
    fn version(&self) -> String {
        self.version.clone()
    }

    /// java: the record accessor `operator()`
    fn operator(&self) -> ComparisonOperator {
        self.operator
    }
}
