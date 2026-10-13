//! Ported from net.azureaaron.dandelion.impl.patching.ConfigPatcher.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::{self, ConfigPatch, PatchCodec};
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::{self, PatchPredicate, PredicateCodec};
use crate::net::azureaaron::dandelion::r#impl::patching::{boolean_patch, date_predicate, enum_patch, minecraft_version_predicate, mod_version_predicate};
use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::extra_codecs::LateBoundIdMapper;

/// java: public static final LateBoundIdMapper<Identifier, MapCodec<? extends ConfigPatch>> PATCH_ID_MAPPER
///
/// Filled by the class's static initialiser with the built-in patch types.
pub static PATCH_ID_MAPPER: compat::MainThread<LateBoundIdMapper<Identifier, PatchCodec>> = compat::MainThread::new(|| {
    let mut value = LateBoundIdMapper::new();
    // Register built-in patch types
    value.put(config_patch::OF_BOOLEAN_ID(), boolean_patch::MAP_CODEC.get().clone());
    value.put(config_patch::OF_ENUM_ID(), enum_patch::MAP_CODEC.get().clone());
    value
});

/// java: public static final LateBoundIdMapper<Identifier, MapCodec<? extends PatchPredicate>> PREDICATE_ID_MAPPER
///
/// rust: the port's `LateBoundIdMapper.codec` snapshots the map when the codec is built (GAPS_D.md D5), so a
/// predicate registered after the first patch was decoded is not seen.
pub static PREDICATE_ID_MAPPER: compat::MainThread<LateBoundIdMapper<Identifier, PredicateCodec>> = compat::MainThread::new(|| {
    let mut value = LateBoundIdMapper::new();
    // Register built-in predicate types
    value.put(patch_predicate::OF_DATE_ID(), date_predicate::MAP_CODEC.get().clone());
    value.put(patch_predicate::OF_MINECRAFT_VERSION_ID(), minecraft_version_predicate::MAP_CODEC.get().clone());
    value.put(patch_predicate::OF_MOD_VERSION_ID(), mod_version_predicate::MAP_CODEC.get().clone());
    value
});

/// java: protected static final Codec<List<PatchPredicate>> PATCH_PREDICATES_CODEC = PREDICATE_ID_MAPPER
/// .codec(Identifier.CODEC).dispatch("id", PatchPredicate::codec, Function.identity()).listOf();
pub static PATCH_PREDICATES_CODEC: compat::MainThread<CodecRef<Vec<Arc<dyn PatchPredicate>>>> = compat::MainThread::new(|| {
    let key_codec = PREDICATE_ID_MAPPER.get().codec(Identifier::codec());
    codec::list(codec::dispatch_keyed(key_codec, "id", |predicate: &Arc<dyn PatchPredicate>| predicate.codec(), |c: &PredicateCodec| c.0.clone()))
});

/// java: public class ConfigPatcher
pub struct ConfigPatcher {}

impl ConfigPatcher {
    /// java: public static void applyPatches(Object config, List<ConfigPatch> patches)
    ///
    /// The config's reflection is its class's generated `patch` form (`compat::ClassOf::patch`).
    pub fn apply_patches<T: 'static>(config: &compat::Shared<T>, config_class: &compat::ClassOf<T>, patches: &[Arc<dyn ConfigPatch>]) {
        for patch in patches {
            // java: patch.predicates().stream().map(PatchPredicate::test).reduce(true, (a, b) -> a & b) - every test runs
            let applicable = patch.predicates().iter().map(|p| p.test()).fold(true, |a, b| a & b);
            if applicable {
                ConfigPatcher::apply_patch(config, config_class, &**patch);
            }
        }
    }

    /// java: @SuppressWarnings({ "rawtypes", "unchecked" }) private static void applyPatch(Object root, ConfigPatch patch)
    ///
    /// Java splits the path on `.`, walks every field but the last by reflection and sets the last one when its type
    /// matches the patch (`boolean` for `OfBoolean`, an enum for `OfEnum`); a missing field or a mismatched type
    /// returns silently. The class's `patch` form does that walk and answers `false` where reflection would fail.
    fn apply_patch<T: 'static>(root: &compat::Shared<T>, config_class: &compat::ClassOf<T>, patch: &dyn ConfigPatch) {
        let path = patch.path();
        let split: Vec<String> = compat::regex::split(&path, "\\.");
        let segments: Vec<&str> = split.iter().map(|s| s.as_str()).collect();
        let value = if let Some(boolean_patch) = patch.as_of_boolean() {
            compat::PatchValue::Bool(boolean_patch.value())
        } else if let Some(enum_patch) = patch.as_of_enum() {
            compat::PatchValue::Enum(enum_patch.value())
        } else {
            // default -> {}
            return;
        };
        compat::write(root, |config| {
            // the field did not exist, or its type did not match: nothing is written
            let _ = (config_class.patch)(config, &segments, &value);
        });
    }
}
