//! Ported from net.azureaaron.dandelion.impl.utils.CodecUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::awt::color::Color;

// TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
//   java: public static final Codec<Color> COLOUR_CODEC = Codec.INT.xmap(argb -> new Color(argb, true), Color::getRGB);
/// java: public static final Codec<Color> COLOUR_CODEC = Codec.INT.xmap(argb -> new Color(argb, true), Color::getRGB);
pub static COLOUR_CODEC: compat::MainThread<CodecRef<Color>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COLOUR_CODEC` - see the TODO above"));
/// java: public static final Codec<SemanticVersion> SEMANTIC_VERSION_CODEC = Codec.STRING.comapFlatMap(version -> { try { return DataResult.success(SemanticVersion.parse(version)); } catch (VersionParsingExcep
pub static SEMANTIC_VERSION_CODEC: compat::MainThread<CodecRef<mcrust::mcrust::modding::version::SemanticVersion>> = compat::MainThread::new(|| codec::comap_flat_map(mcrust::com::mojang::serialization::codec::string_codec(), |version| {
    // java: try {..} catch (VersionParsingException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        return DataResult::success(mcrust::mcrust::modding::version::SemanticVersion::parse(&compat::jstring(&version)).expect("VersionParsingException"));
    })) {
        Ok(__v) => return __v,
        Err(_) => {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<String>`)
            // rust: return DataResult::error(|| format!("Failed to parse semantic version from string: {}", compat::jstring(&version)));
        }
    }
}, |__a0| compat::jstring(&__a0)));

/// java: public final class CodecUtils
#[derive(Clone)]
pub struct CodecUtils {}

impl CodecUtils {
    /// java: private CodecUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
