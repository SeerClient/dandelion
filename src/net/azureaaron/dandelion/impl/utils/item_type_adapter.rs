//! Ported from net.azureaaron.dandelion.impl.utils.ItemTypeAdapter.
//!
//! An `Item` value is `compat::JavaRef<Item>` (as YACL's `ItemTypeAdapter`). Gson's `Type` / context parameters are
//! unused by the bodies and are not carried.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use mcrust::net::minecraft::core::registries::built_in_registries;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::items;

/// java: public class ItemTypeAdapter implements JsonSerializer<Item>, JsonDeserializer<Item>
#[derive(Clone, Default)]
pub struct ItemTypeAdapter {}

impl ItemTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public Item deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context)
    pub fn deserialize(&self, json: &serde_json::Value) -> compat::JavaRef<Item> {
        // java: BuiltInRegistries.ITEM.getValue(Identifier.parse(json.getAsString().toLowerCase(Locale.CANADA))) - the
        // defaulted registry answers air for an unknown id
        let id = Identifier::parse(&compat::json::get_as_string(json).to_lowercase());
        ItemRegistryHelper::get_from_registry(&id).unwrap_or_else(|| compat::JavaRef::new(items::air()))
    }

    /// java: @Override public JsonElement serialize(Item src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &compat::JavaRef<Item>) -> serde_json::Value {
        // java: new JsonPrimitive(BuiltInRegistries.ITEM.getKey(src).toString())
        serde_json::Value::String(built_in_registries::item().get_key_defaulted(&src.erased_obj()).to_string())
    }
}
