//! Ported from net.azureaaron.dandelion.impl.utils.StyleTypeAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::style::Style;

/// java: public class StyleTypeAdapter implements JsonSerializer<Style>, JsonDeserializer<Style>
#[derive(Clone)]
pub struct StyleTypeAdapter {}

impl StyleTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Style deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> Style {
        // TODO(fabric2seer): field `CODEC` of a value of unknown type
        //   java: return Style.Serializer.CODEC.parse(JsonOps.INSTANCE, json).result().orElse(Style.EMPTY);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public JsonElement serialize(Style src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Style, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        // TODO(fabric2seer): field `CODEC` of a value of unknown type
        //   java: return Style.Serializer.CODEC.encodeStart(JsonOps.INSTANCE, src).result().orElse(JsonNull.INSTANCE);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
