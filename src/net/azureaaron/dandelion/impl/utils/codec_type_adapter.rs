//! Ported from net.azureaaron.dandelion.impl.utils.CodecTypeAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public record CodecTypeAdapter<T>(Codec<T> codec) implements JsonSerializer<T>, JsonDeserializer<T>
pub struct CodecTypeAdapter<T> {
    /// java: Codec<T> codec
    pub codec: CodecRef<T>,
}

impl<T: Clone + 'static> CodecTypeAdapter<T> {
    /// java: the canonical record constructor
    pub fn new(mut codec: CodecRef<T>) -> Self {
        Self { codec }
    }

    /// java: the record accessor `codec()`
    pub fn codec(&self) -> CodecRef<T> {
        self.codec.clone()
    }

    /// java: @Override public T deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> T {
        // TODO(fabric2seer): `new JsonParseException(..)`
        //   java: return this.codec.parse(JsonOps.INSTANCE, json).getOrThrow(JsonParseException::new);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public JsonElement serialize(T src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &T, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        return (self.codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), src)).get_or_throw().unwrap();
    }

}
