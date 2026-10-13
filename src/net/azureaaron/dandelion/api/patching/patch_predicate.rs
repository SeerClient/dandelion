//! Ported from net.azureaaron.dandelion.api.patching.PatchPredicate.
//!
//! A predicate that will be used in determining whether a `ConfigPatch` should be applied. Predicates are records
//! decoded by a codec, so they are `Send + Sync` (the record codec builder's bound).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::ComparisonOperator;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::mcrust::modding::version::SemanticVersion;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: Identifier ID = Dandelion.id("predicate/date");
#[allow(non_snake_case)]
pub fn OF_DATE_ID() -> Identifier {
    crate::entry::dandelion::id("predicate/date")
}

/// java: Identifier ID = Dandelion.id("predicate/minecraft_version");
#[allow(non_snake_case)]
pub fn OF_MINECRAFT_VERSION_ID() -> Identifier {
    crate::entry::dandelion::id("predicate/minecraft_version")
}

/// java: Identifier ID = Dandelion.id("predicate/mod_version");
#[allow(non_snake_case)]
pub fn OF_MOD_VERSION_ID() -> Identifier {
    crate::entry::dandelion::id("predicate/mod_version")
}

/// java: `MapCodec<? extends PatchPredicate>` - a predicate record's codec seen as the codec of the interface
/// (`map_codec::upcast`), compared by identity (the `LateBoundIdMapper`'s `BiMap` key).
#[derive(Clone)]
pub struct PredicateCodec(pub MapCodecRef<Arc<dyn PatchPredicate>>);

impl PartialEq for PredicateCodec {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(Arc::as_ptr(&self.0) as *const (), Arc::as_ptr(&other.0) as *const ())
    }
}

/// java: public interface PatchPredicate
pub trait PatchPredicate: compat::DynAny + Send + Sync {
    /// java: MapCodec<? extends PatchPredicate> codec();
    fn codec(&self) -> PredicateCodec;

    /// java: boolean test();
    ///
    /// `true` if the predicate's condition was satisfied, otherwise `false`.
    fn test(&self) -> bool;
}

/// Java's `Object.equals` for an interface reference the codecs compare (`optionalFieldOf(.., List.of())`): identity.
impl PartialEq for dyn PatchPredicate {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self as *const dyn PatchPredicate as *const (), other as *const dyn PatchPredicate as *const ())
    }
}

/// java: interface OfDate extends PatchPredicate
pub trait OfDate: PatchPredicate {
    /// java: OptionalLong from();
    fn from(&self) -> Option<i64>;

    /// java: OptionalLong to();
    fn to(&self) -> Option<i64>;
}

/// java: interface OfMinecraftVersion extends PatchPredicate
pub trait OfMinecraftVersion: PatchPredicate {
    /// java: String version();
    fn version(&self) -> String;

    /// java: ComparisonOperator operator();
    fn operator(&self) -> ComparisonOperator;
}

/// java: interface OfModVersion extends PatchPredicate
pub trait OfModVersion: PatchPredicate {
    /// java: String modId();
    fn mod_id(&self) -> String;

    /// java: SemanticVersion version();
    fn version(&self) -> SemanticVersion;

    /// java: ComparisonOperator operator();
    fn operator(&self) -> ComparisonOperator;
}
