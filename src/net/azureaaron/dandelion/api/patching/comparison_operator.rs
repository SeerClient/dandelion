//! Ported from net.azureaaron.dandelion.api.patching.ComparisonOperator.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::mcrust::modding::version::Version;
use mcrust::net::minecraft::util::string_representable;

/// java: public static final Codec<ComparisonOperator> CODEC = StringRepresentable.fromEnum(ComparisonOperator::values);
pub static CODEC: LazyLock<CodecRef<ComparisonOperator>> =
    LazyLock::new(|| string_representable::from_enum(ComparisonOperator::values(), |e: &ComparisonOperator| e.get_serialized_name()));

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
    pub fn compare(&self, version1: &Version, version2: &Version) -> bool {
        let c = compare_versions(version1, version2);
        match self {
            ComparisonOperator::EqualTo => c == 0,
            ComparisonOperator::NotEqualTo => c != 0,
            ComparisonOperator::GreaterThan => c > 0,
            ComparisonOperator::GreaterThanOrEqualTo => c >= 0,
            ComparisonOperator::LessThan => c < 0,
            ComparisonOperator::LessThanOrEqualTo => c <= 0,
        }
    }
}

/// Fabric's `Version.compareTo(Version)`: two semantic versions compare by semver precedence; otherwise
/// (`StringVersion`, or a semantic version against a string one) by `getFriendlyString()` - `String.compareTo`.
fn compare_versions(version1: &Version, version2: &Version) -> i32 {
    match (version1.as_semantic(), version2.as_semantic()) {
        (Some(a), Some(b)) => compat::ord_int(a.cmp(b)),
        _ => compat::compare_to(&version1.to_string(), &version2.to_string()),
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
