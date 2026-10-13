//! Ported from net.azureaaron.dandelion.impl.utils.CodecUtils.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::awt::color::Color;
use mcrust::mcrust::modding::version::SemanticVersion;

/// java: public static final Codec<Color> COLOUR_CODEC = Codec.INT.xmap(argb -> new Color(argb, true), Color::getRGB);
pub static COLOUR_CODEC: LazyLock<CodecRef<Color>> =
    LazyLock::new(|| codec::xmap(codec::int_codec(), |argb: i32| Color::from_rgba(argb, true), |color: &Color| color.get_rgb()));

/// java: public static final Codec<SemanticVersion> SEMANTIC_VERSION_CODEC = Codec.STRING.comapFlatMap(version -> {
/// try { return DataResult.success(SemanticVersion.parse(version)); } catch (VersionParsingException _) {
/// return DataResult.error(() -> "Failed to parse semantic version from string: " + version); } },
/// SemanticVersion::toString);
pub static SEMANTIC_VERSION_CODEC: LazyLock<CodecRef<SemanticVersion>> = LazyLock::new(|| {
    codec::comap_flat_map(
        codec::string_codec(),
        |version: String| match SemanticVersion::parse(&version) {
            Ok(parsed) => DataResult::success(parsed),
            Err(_) => DataResult::error(Arc::new(move || format!("Failed to parse semantic version from string: {version}"))),
        },
        |version: &SemanticVersion| version.to_string(),
    )
});

/// java: public final class CodecUtils
pub struct CodecUtils {}
