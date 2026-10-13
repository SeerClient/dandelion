//! Ported from net.azureaaron.dandelion.impl.utils.CodecTypeAdapter.
//!
//! Creates a Gson type adapter from a `Codec`. Gson's tree is the mod's `serde_json::Value`, `JsonOps`' the port's
//! (`compat::json_to_port` / `json_from_port`). Gson's `Type` / context parameters are unused and not carried.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::com::mojang::serialization::json_ops::JsonOps;

/// java: public record CodecTypeAdapter<T>(Codec<T> codec) implements JsonSerializer<T>, JsonDeserializer<T>
pub struct CodecTypeAdapter<T> {
    /// java: Codec<T> codec
    pub codec: CodecRef<T>,
}

impl<T: 'static> CodecTypeAdapter<T> {
    /// java: the canonical record constructor
    pub fn new(codec: CodecRef<T>) -> Self {
        Self { codec }
    }

    /// java: the record accessor `codec()`
    pub fn codec(&self) -> CodecRef<T> {
        self.codec.clone()
    }

    /// java: @Override public T deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value) -> T {
        // java: this.codec.parse(JsonOps.INSTANCE, json).getOrThrow(JsonParseException::new);
        match self.codec.parse(&JsonOps::instance(), &compat::json_to_port(json)).get_or_throw() {
            Ok(value) => value,
            Err(message) => panic!("JsonParseException: {message}"),
        }
    }

    /// java: @Override public JsonElement serialize(T src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &T) -> serde_json::Value {
        // java: this.codec.encodeStart(JsonOps.INSTANCE, src).getOrThrow();
        match self.codec.encode_start(&JsonOps::instance(), src).get_or_throw() {
            Ok(value) => compat::json_from_port(&value),
            Err(message) => panic!("IllegalStateException: {message}"),
        }
    }
}
