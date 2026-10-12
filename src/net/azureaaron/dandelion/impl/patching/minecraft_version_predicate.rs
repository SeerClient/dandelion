//! Ported from net.azureaaron.dandelion.impl.patching.MinecraftVersionPredicate by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::ComparisonOperator;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::OfMinecraftVersion;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::shared_constants;

/// java: protected static final MapCodec<MinecraftVersionPredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group( Codec.STRING.fieldOf("version").forGetter(MinecraftVersionPredicate::vers
pub static MAP_CODEC: compat::MainThread<MapCodecRef<MinecraftVersionPredicate>> = compat::MainThread::new(|| record_codec_builder::map_codec(record_codec_builder::apply2(record_codec_builder::of(|o: &MinecraftVersionPredicate| o.version(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "version")), record_codec_builder::of(|o: &MinecraftVersionPredicate| o.operator(), codec::field_of(comparison_operator::CODEC.get_mut().clone(), "operator")), move |__a0, __a1| MinecraftVersionPredicate::new(__a0.clone(), __a1.clone()))));

/// java: public record MinecraftVersionPredicate(String version, ComparisonOperator operator) implements PatchPredicate.OfMinecraftVersion
#[derive(Clone, Debug, PartialEq)]
pub struct MinecraftVersionPredicate {
    /// java: String version
    pub version: String,
    /// java: ComparisonOperator operator
    pub operator: ComparisonOperator,
}

impl MinecraftVersionPredicate {
    /// java: the canonical record constructor
    pub fn new(mut version: String, mut operator: ComparisonOperator) -> Self {
        Self { version, operator }
    }

    /// java: the record accessor `version()`
    pub fn version(&self) -> String {
        self.version.clone()
    }

    /// java: the record accessor `operator()`
    pub fn operator(&self) -> ComparisonOperator {
        self.operator
    }

}

impl OfMinecraftVersion for MinecraftVersionPredicate {
}

impl PatchPredicate for MinecraftVersionPredicate {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public MapCodec<? extends PatchPredicate> codec() (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        MinecraftVersionPredicate::codec_dyn
    }

    /// java: @Override public boolean test() (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> bool {
        MinecraftVersionPredicate::test_dyn
    }

}

impl MinecraftVersionPredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    pub fn codec_impl(&self) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        return MAP_CODEC.get_mut().clone();
    }

    /// MinecraftVersionPredicate.codec as the dispatch of PatchPredicate.codec
    pub fn codec_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.codec_impl())
    }

    /// java: @Override public boolean test()
    pub fn test_impl(&self) -> bool {
        let mut current_version_id: String = shared_constants::get_current_version().id().to_string();
        // TODO(fabric2seer): static call McVersionLookup.normalizeVersion(..)
        //   java: String normalizedCurrent = McVersionLookup.normalizeVersion(currentVersionId, McVersionLookup.getRelease(currentVersionId));
        let mut normalized_current: String = todo!("fabric2seer: the initialiser of `normalizedCurrent` (TODO above)");
        // TODO(fabric2seer): static call McVersionLookup.normalizeVersion(..)
        //   java: String normalizedTarget = McVersionLookup.normalizeVersion(this.version(), McVersionLookup.getRelease(this.version()));
        let mut normalized_target: String = todo!("fabric2seer: the initialiser of `normalizedTarget` (TODO above)");
        // java: try {..} catch (VersionParsingException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut current_version: mcrust::mcrust::modding::version::SemanticVersion = mcrust::mcrust::modding::version::SemanticVersion::parse(&normalized_current).expect("VersionParsingException");
            let mut target_version: mcrust::mcrust::modding::version::SemanticVersion = mcrust::mcrust::modding::version::SemanticVersion::parse(&normalized_target).expect("VersionParsingException");
            return self.operator().compare(&current_version, &target_version);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
            }
        }
        return false;
    }

    /// MinecraftVersionPredicate.test as the dispatch of PatchPredicate.test
    pub fn test_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> bool {
        compat::read_as::<Self, _, _>(__this, |__s| __s.test_impl())
    }

}
