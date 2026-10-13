//! Ported from net.azureaaron.dandelion.impl.patching.DatePredicate.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::{OfDate, PatchPredicate, PredicateCodec};
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec;
use mcrust::net::minecraft::util::extra_codecs;

/// java: protected static final MapCodec<DatePredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group(
/// ExtraCodecs.asOptionalLong(Codec.LONG.optionalFieldOf("from")).forGetter(DatePredicate::from),
/// ExtraCodecs.asOptionalLong(Codec.LONG.optionalFieldOf("to")).forGetter(DatePredicate::to)).apply(instance, DatePredicate::new));
///
/// Held upcast to `MapCodec<? extends PatchPredicate>` (`map_codec::upcast`): the same object `codec()` answers and
/// the id mapper holds.
pub static MAP_CODEC: compat::MainThread<PredicateCodec> = compat::MainThread::new(|| {
    let record = record_codec_builder::map_codec(record_codec_builder::apply2(
        record_codec_builder::of(|o: &DatePredicate| o.from, extra_codecs::as_optional_long(codec::optional_field_of_opt(codec::long_codec(), "from"))),
        record_codec_builder::of(|o: &DatePredicate| o.to, extra_codecs::as_optional_long(codec::optional_field_of_opt(codec::long_codec(), "to"))),
        |from, to| DatePredicate::new(from, to),
    ));
    PredicateCodec(map_codec::upcast(
        record,
        |p: DatePredicate| Arc::new(p) as Arc<dyn PatchPredicate>,
        |p: &dyn PatchPredicate| p.dyn_any().downcast_ref::<DatePredicate>(),
    ))
});

/// java: public record DatePredicate(OptionalLong from, OptionalLong to) implements PatchPredicate.OfDate
#[derive(Clone, Debug, PartialEq)]
pub struct DatePredicate {
    /// java: OptionalLong from
    pub from: Option<i64>,
    /// java: OptionalLong to
    pub to: Option<i64>,
}

impl DatePredicate {
    /// java: the canonical record constructor
    pub fn new(from: Option<i64>, to: Option<i64>) -> Self {
        Self { from, to }
    }
}

impl PatchPredicate for DatePredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    fn codec(&self) -> PredicateCodec {
        MAP_CODEC.get().clone()
    }

    /// java: @Override public boolean test()
    fn test(&self) -> bool {
        let mut result = true;
        let now = compat::millis();
        if let Some(from) = self.from {
            result &= now >= from;
        }
        if let Some(to) = self.to {
            result &= now <= to;
        }
        result
    }
}

impl OfDate for DatePredicate {
    /// java: the record accessor `from()`
    fn from(&self) -> Option<i64> {
        self.from
    }

    /// java: the record accessor `to()`
    fn to(&self) -> Option<i64> {
        self.to
    }
}
