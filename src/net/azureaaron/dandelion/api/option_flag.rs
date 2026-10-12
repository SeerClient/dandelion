//! Ported from net.azureaaron.dandelion.api.OptionFlag by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): `.delayTextureReload(..)` on a value of unknown type
//   java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
/// java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
pub static ASSET_RELOAD: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ASSET_RELOAD` - see the TODO above"));
// TODO(fabric2seer): field `levelExtractor` of a value of unknown type
//   java: OptionFlag RELOAD_CHUNKS = client -> client.levelExtractor.allChanged();
/// java: OptionFlag RELOAD_CHUNKS = client -> client.levelExtractor.allChanged();
pub static RELOAD_CHUNKS: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `RELOAD_CHUNKS` - see the TODO above"));

/// java: @FunctionalInterface public interface OptionFlag extends Consumer<Minecraft>
pub trait OptionFlag: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
