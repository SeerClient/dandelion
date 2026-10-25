# GAPS_D - Dandelion fix-up

Each row: the Java member, what the port / crate has now, what is needed, sites. Sites carry
`rust: port gap (GAPS_D.md Dn)` or a `todo!(".. (GAPS_D.md Dn)")` naming the row.

| # | Java | Port / crate now | Needed | Sites |
|---|---|---|---|---|
| D1 | MoulConfig (`io.github.notenoughupdates.moulconfig`, shaded by Dandelion) | **CLOSED** - ported as the library crate `mods-src/moulconfig` (MoulConfig 4.7.2, `modern-26.2`): processor model, `MoulConfigEditor`, the editors Dandelion builds, the component tree, the render/platform layer, `MoulConfigScreenComponent`. `impl/moulconfig/**` and the two mixins are wired to it; `unresolved.rs` holds no MoulConfig placeholder; the textures are in `assets/moulconfig/` (shaded, as in Java). The library's own not-ported list (annotation processor, XML layer, managed config, NinePatch) is in its `lib.rs` - none of it has a caller here | - | `impl/moulconfig/**`, `mixins/moul_config_screen_component_mixin.rs`, `mixins/gui_option_editor_dropdown_accessor.rs` |
| D3 | `Minecraft.options.save()` writing every `KeyMapping`'s live key (`Options.processOptions`) | the port's `options::save` writes the options map; a `key_mapping::set_key` from mod code (a `KeyMappingOption` rebound in YACL, `DandelionKeyMappingEditor.setKey`) never reaches the map, so the rebind is live but not saved to options.txt | `Options.save` refreshing `key_<name>` from `KeyMapping.ALL` (or a `key_mapping::set_key` that updates the options data) - port | `impl/config_manager_impl.rs` (`save_all`) |
| D4 | Fabric Loader's `McVersionLookup.normalizeVersion` / `getRelease` | not in the port (`mcrust::modding::version` has `SemanticVersion` / `Version` only) | the normaliser - port (`mcrust::modding::version`) | `impl/patching/minecraft_version_predicate.rs` (`test` - a `todo!`) |
| D5 | `ExtraCodecs.LateBoundIdMapper.codec(..)` is LATE-bound (reads the BiMap at decode time) | the port's `codec()` copies the entries when the codec is built | a codec over a shared map - port | `impl/patching/config_patcher.rs` (`PATCH_PREDICATES_CODEC` is built on first use, so a `PatchPredicateRegistry.register` after the first patch decode is not seen) |

## Not gaps (decided, documented at the site)

* `Option<?>` is `Arc<dyn AnyOption>`; the two generic adapter calls Java makes through a wildcard
  (`toYaclOption`, `createEditableOptionFactory`) and the `instanceof ButtonOption/KeyMappingOption/LabelOption` tests
  are `AnyOption` methods; `switch (group) { case ListOption .. }` is `OptionGroup::to_yacl_option_group`.
* The sealed `Controller<T>` switch is `Controller::kind() -> ControllerKind`; the enum arm runs in
  `EnumControllerImpl<T: JavaEnum>` (`AnyEnumController`). Java's unchecked generic casts are `compat::java_cast`
  (a mismatch panics as `ClassCastException`).
* `Class<T> type()` of an option is a `compat::ClassOf<T>` named after `T` whose reflective members panic
  (`ReflectionUtils::get_actual_class`); `OptionImpl.checkType` compares `TypeId`s.
* `Consumer<Screen>` (button action) is `ScreenConsumer`; the screen handed in is `compat::ScreenRef::Current`.
* A list option's entry binding reads/writes element `i` through the list binding (Java mutates the list the getter
  returned).
* `IntegerControllerBuilderImpl.min(..)` stores nothing - the Java body is `return this;` (kept).
