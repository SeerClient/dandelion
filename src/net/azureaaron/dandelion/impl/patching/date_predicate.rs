//! Ported from net.azureaaron.dandelion.impl.patching.DatePredicate by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::OfDate;
use crate::net::azureaaron::dandelion::api::patching::patch_predicate::PatchPredicate;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::util::extra_codecs;

/// java: protected static final MapCodec<DatePredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group( ExtraCodecs.asOptionalLong(Codec.LONG.optionalFieldOf("from")).forGetter(DatePredicat
pub static MAP_CODEC: compat::MainThread<MapCodecRef<DatePredicate>> = compat::MainThread::new(|| record_codec_builder::map_codec(record_codec_builder::apply2(record_codec_builder::of(|o: &DatePredicate| o.from(), extra_codecs::as_optional_long(codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::long_codec(), "from"))), record_codec_builder::of(|o: &DatePredicate| o.to(), extra_codecs::as_optional_long(codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::long_codec(), "to"))), move |__a0, __a1| DatePredicate::new(__a0.clone(), __a1.clone()))));

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
    pub fn new(mut from: Option<i64>, mut to: Option<i64>) -> Self {
        Self { from, to }
    }

    /// java: the record accessor `from()`
    pub fn from(&self) -> Option<i64> {
        self.from.clone()
    }

    /// java: the record accessor `to()`
    pub fn to(&self) -> Option<i64> {
        self.to.clone()
    }

}

impl OfDate for DatePredicate {
}

impl PatchPredicate for DatePredicate {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public MapCodec<? extends PatchPredicate> codec() (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        DatePredicate::codec_dyn
    }

    /// java: @Override public boolean test() (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> bool {
        DatePredicate::test_dyn
    }

}

impl DatePredicate {
    /// java: @Override public MapCodec<? extends PatchPredicate> codec()
    pub fn codec_impl(&self) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        return MAP_CODEC.get_mut().clone();
    }

    /// DatePredicate.codec as the dispatch of PatchPredicate.codec
    pub fn codec_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.codec_impl())
    }

    /// java: @Override public boolean test()
    pub fn test_impl(&self) -> bool {
        let mut result: bool = true;
        let mut now: i64 = compat::millis();
        if self.from().is_some() {
            result = ((result & (now >= self.from().unwrap()))) as bool;
        }
        if self.to().is_some() {
            result = ((result & (now <= self.to().unwrap()))) as bool;
        }
        return result;
    }

    /// DatePredicate.test as the dispatch of PatchPredicate.test
    pub fn test_dyn(__this: &compat::Shared<dyn PatchPredicate>) -> bool {
        compat::read_as::<Self, _, _>(__this, |__s| __s.test_impl())
    }

}
