//! Ported from net.azureaaron.dandelion.api.patching.ComparisonOperator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<ComparisonOperator> CODEC = StringRepresentable.fromEnum(ComparisonOperator::values);
/// java: public static final Codec<ComparisonOperator> CODEC = StringRepresentable.fromEnum(ComparisonOperator::values);
pub static CODEC: compat::MainThread<CodecRef<ComparisonOperator>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public enum ComparisonOperator implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComparisonOperator {
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

impl ComparisonOperator {
    pub const VALUES: [ComparisonOperator; 6] = [ComparisonOperator::EqualTo, ComparisonOperator::NotEqualTo, ComparisonOperator::GreaterThan, ComparisonOperator::GreaterThanOrEqualTo, ComparisonOperator::LessThan, ComparisonOperator::LessThanOrEqualTo];

    /// java: values()
    pub fn values() -> Vec<ComparisonOperator> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ComparisonOperator::EqualTo => "EQUAL_TO",
            ComparisonOperator::NotEqualTo => "NOT_EQUAL_TO",
            ComparisonOperator::GreaterThan => "GREATER_THAN",
            ComparisonOperator::GreaterThanOrEqualTo => "GREATER_THAN_OR_EQUAL_TO",
            ComparisonOperator::LessThan => "LESS_THAN",
            ComparisonOperator::LessThanOrEqualTo => "LESS_THAN_OR_EQUAL_TO",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ComparisonOperator {
        match name {
            "EQUAL_TO" => ComparisonOperator::EqualTo,
            "NOT_EQUAL_TO" => ComparisonOperator::NotEqualTo,
            "GREATER_THAN" => ComparisonOperator::GreaterThan,
            "GREATER_THAN_OR_EQUAL_TO" => ComparisonOperator::GreaterThanOrEqualTo,
            "LESS_THAN" => ComparisonOperator::LessThan,
            "LESS_THAN_OR_EQUAL_TO" => ComparisonOperator::LessThanOrEqualTo,
            _ => panic!("IllegalArgumentException: No enum constant ComparisonOperator.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.name().to_string();
    }

    /// java: public boolean compare(Version version1, Version version2)
    pub fn compare(&self, version1: &mcrust::mcrust::modding::version::Version, version2: &mcrust::mcrust::modding::version::Version) -> bool {
        // TODO(fabric2seer): `Version.compareTo` (the loader's Version)
        //   java: return switch (this) {
        //   java: 			case EQUAL_TO -> version1.compareTo(version2) == 0;
        //   java: 			case NOT_EQUAL_TO -> version1.compareTo(version2) != 0;
        //   java: 			case GREATER_THAN -> version1.compareTo(version2) > 0;
        //   java: 			case GREATER_THAN_OR_EQUAL_TO -> version1.compareTo(version2) >= 0;
        //   java: 			case LESS_THAN -> version1.compareTo(version2) < 0;
        //   java: 			case LESS_THAN_OR_EQUAL_TO -> version1.compareTo(version2) <= 0;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
