//! Ported from net.azureaaron.dandelion.impl.patching.ConfigPatcher by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use crate::net::azureaaron::dandelion::r#impl::patching::boolean_patch;
use crate::net::azureaaron::dandelion::r#impl::patching::date_predicate;
use crate::net::azureaaron::dandelion::r#impl::patching::enum_patch;
use crate::net::azureaaron::dandelion::r#impl::patching::minecraft_version_predicate;
use crate::net::azureaaron::dandelion::r#impl::patching::mod_version_predicate;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::extra_codecs::LateBoundIdMapper;

/// java: public static final LateBoundIdMapper<Identifier, MapCodec<? extends ConfigPatch>> PATCH_ID_MAPPER = new LateBoundIdMapper<>();
pub static PATCH_ID_MAPPER: compat::MainThread<LateBoundIdMapper<Identifier, MapCodecRef<compat::Shared<dyn ConfigPatch>>>> = compat::MainThread::new(|| {
    let mut value = LateBoundIdMapper::new();
    value.put(config_patch::OF_BOOLEAN_ID.get_mut().clone(), boolean_patch::MAP_CODEC.get_mut().clone());
    value.put(config_patch::OF_ENUM_ID.get_mut().clone(), enum_patch::MAP_CODEC.get_mut().clone());
    value
});
/// java: public static final LateBoundIdMapper<Identifier, MapCodec<? extends PatchPredicate>> PREDICATE_ID_MAPPER = new LateBoundIdMapper<>();
pub static PREDICATE_ID_MAPPER: compat::MainThread<LateBoundIdMapper<Identifier, MapCodecRef<compat::Shared<dyn PatchPredicate>>>> = compat::MainThread::new(|| {
    let mut value = LateBoundIdMapper::new();
    value.put(patch_predicate::OF_DATE_ID.get_mut().clone(), date_predicate::MAP_CODEC.get_mut().clone());
    value.put(patch_predicate::OF_MINECRAFT_VERSION_ID.get_mut().clone(), minecraft_version_predicate::MAP_CODEC.get_mut().clone());
    value.put(patch_predicate::OF_MOD_VERSION_ID.get_mut().clone(), mod_version_predicate::MAP_CODEC.get_mut().clone());
    value
});
// TODO(fabric2seer): `.codec(..)` on a E
//   java: protected static final Codec<List<PatchPredicate>> PATCH_PREDICATES_CODEC = PREDICATE_ID_MAPPER.codec(Identifier.CODEC).dispatch("id", PatchPredicate::codec, Function.identity())
//   java: 			.listOf();
/// java: protected static final Codec<List<PatchPredicate>> PATCH_PREDICATES_CODEC = PREDICATE_ID_MAPPER.codec(Identifier.CODEC).dispatch("id", PatchPredicate::codec, Function.identity()) .listOf();
pub static PATCH_PREDICATES_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<dyn PatchPredicate>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PATCH_PREDICATES_CODEC` - see the TODO above"));

/// java: public class ConfigPatcher
#[derive(Clone)]
pub struct ConfigPatcher {}

impl ConfigPatcher {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void applyPatches(Object config, List<ConfigPatch> patches)
    pub fn apply_patches(config: &compat::Object, patches: &Vec<compat::Shared<dyn ConfigPatch>>) {
        for patch in patches.iter() {
            let mut applicable: bool = ({ let __f = compat::dispatch(patch, |__t| __t.predicates()); __f(patch) }).into_iter().map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.test()); __f(&__a0) })).fold(true, |a, b| (a & b));
            if applicable {
                ConfigPatcher::apply_patch(config, patch);
            }
        }
    }

    /// java: @SuppressWarnings(
    pub fn apply_patch(root: &compat::Object, patch: &compat::Shared<dyn ConfigPatch>) {
        let mut split: Vec<String> = compat::regex::split(&({ let __f = compat::dispatch(patch, |__t| __t.path()); __f(patch) }), "\\.");
        let mut target_field_name: String = split[((split.len() as i32).wrapping_sub(1)) as usize].clone();
        let mut objects2_traverse: Vec<String> = (0..((split.len() as i32).wrapping_sub(1)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        objects2_traverse[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize].clone_from_slice(&split[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize]);
        let mut current_node: compat::Object = root.clone();
        for object_name in objects2_traverse.iter() {
            // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.getDeclaredField(..)` on a Class
                //   java: Field field = currentNode.getClass().getDeclaredField(objectName);
                let mut field: crate::unresolved::Field = todo!("fabric2seer: the initialiser of `field` (TODO above)");
                // TODO(fabric2seer): `.get(..)` on a Field
                //   java: currentNode = field.get(currentNode);
            }));
            if let Err(_) = __result {
                return;
            }
        }
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.getDeclaredField(..)` on a Class
            //   java: Field targetField = currentNode.getClass().getDeclaredField(targetFieldName);
            let mut target_field: crate::unresolved::Field = todo!("fabric2seer: the initialiser of `targetField` (TODO above)");
            // TODO(fabric2seer): not translated (expected '.' at '->' (offset 2725))
            //   java: switch (patch) {
            //   java: 				case ConfigPatch.OfBoolean booleanPatch when targetField.getType() == boolean.class -> targetField.setBoolean(currentNode, booleanPatch.value());
            //   java: 				case ConfigPatch.OfEnum enumPatch when targetField.getType().isEnum() -> {
            //   java: 					Object enumConstant = Enum.valueOf((Class<Enum>) targetField.getType(), enumPatch.value());
            //   java: 					targetField.set(currentNode, enumConstant);
            //   java: 				}
            //   java: 				default -> {}
            //   java: 			}
        }));
        if let Err(_) = __result {
            return;
        }
    }

}
