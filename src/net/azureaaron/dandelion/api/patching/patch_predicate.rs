//! Ported from net.azureaaron.dandelion.api.patching.PatchPredicate by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::comparison_operator::ComparisonOperator;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: Identifier ID = Dandelion.id("predicate/date");
pub static OF_DATE_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("predicate/date"));
/// java: Identifier ID = Dandelion.id("predicate/minecraft_version");
pub static OF_MINECRAFT_VERSION_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("predicate/minecraft_version"));
/// java: Identifier ID = Dandelion.id("predicate/mod_version");
pub static OF_MOD_VERSION_ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("predicate/mod_version"));

/// java: public interface PatchPredicate
pub trait PatchPredicate: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: MapCodec<? extends PatchPredicate> codec(); (dispatch: the body for this class)
    fn codec(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> MapCodecRef<compat::Shared<dyn PatchPredicate>>;

    /// java: boolean test(); (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn PatchPredicate>) -> bool;

}

/// java: interface OfDate extends PatchPredicate
pub trait OfDate: PatchPredicate {
    /// java: OptionalLong from(); (dispatch: the body for this class)
    fn from(&self) -> fn(&compat::Shared<dyn OfDate>) -> Option<i64>;

    /// java: OptionalLong to(); (dispatch: the body for this class)
    fn to(&self) -> fn(&compat::Shared<dyn OfDate>) -> Option<i64>;

}

/// java: interface OfMinecraftVersion extends PatchPredicate
pub trait OfMinecraftVersion: PatchPredicate {
    /// java: String version(); (dispatch: the body for this class)
    fn version(&self) -> fn(&compat::Shared<dyn OfMinecraftVersion>) -> String;

    /// java: ComparisonOperator operator(); (dispatch: the body for this class)
    fn operator(&self) -> fn(&compat::Shared<dyn OfMinecraftVersion>) -> ComparisonOperator;

}

/// java: interface OfModVersion extends PatchPredicate
pub trait OfModVersion: PatchPredicate {
    /// java: String modId(); (dispatch: the body for this class)
    fn mod_id(&self) -> fn(&compat::Shared<dyn OfModVersion>) -> String;

    /// java: SemanticVersion version(); (dispatch: the body for this class)
    fn version(&self) -> fn(&compat::Shared<dyn OfModVersion>) -> mcrust::mcrust::modding::version::SemanticVersion;

    /// java: ComparisonOperator operator(); (dispatch: the body for this class)
    fn operator(&self) -> fn(&compat::Shared<dyn OfModVersion>) -> ComparisonOperator;

}
