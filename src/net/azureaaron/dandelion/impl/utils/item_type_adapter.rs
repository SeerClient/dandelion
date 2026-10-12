//! Ported from net.azureaaron.dandelion.impl.utils.ItemTypeAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: public class ItemTypeAdapter implements JsonSerializer<Item>, JsonDeserializer<Item>
#[derive(Clone)]
pub struct ItemTypeAdapter {}

impl ItemTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Item deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> Arc<Item> {
        return mcrust::net::minecraft::core::registries::built_in_registries::item().get_value_by_identifier(&Identifier::parse(&compat::json::get_as_string(&json).to_lowercase())).unwrap();
    }

    /// java: @Override public JsonElement serialize(Item src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Arc<Item>, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return serde_json::json!(mcrust::net::minecraft::core::registries::built_in_registries::item().get_key(src).unwrap().fmt());
        return todo!("fabric2seer: see the TODO above");
    }

}
