//! Ported from net.azureaaron.dandelion.impl.utils.StyleTypeAdapter.
//!
//! This separate serializer with fallback values is required to avoid errors/crashes in unit tests. Gson's `Type` /
//! context parameters are unused and not carried.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::com::mojang::serialization::json_ops::JsonOps;
use mcrust::net::minecraft::network::chat::style::{self, Style};

/// java: public class StyleTypeAdapter implements JsonSerializer<Style>, JsonDeserializer<Style>
#[derive(Clone, Default)]
pub struct StyleTypeAdapter {}

impl StyleTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public Style deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value) -> Style {
        // java: Style.Serializer.CODEC.parse(JsonOps.INSTANCE, json).result().orElse(Style.EMPTY);
        style::serializer::codec().parse(&JsonOps::instance(), &compat::json_to_port(json)).result().unwrap_or(Style::EMPTY)
    }

    /// java: @Override public JsonElement serialize(Style src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Style) -> serde_json::Value {
        // java: Style.Serializer.CODEC.encodeStart(JsonOps.INSTANCE, src).result().orElse(JsonNull.INSTANCE);
        match style::serializer::codec().encode_start(&JsonOps::instance(), src).result() {
            Some(value) => compat::json_from_port(&value),
            None => serde_json::Value::Null,
        }
    }
}
