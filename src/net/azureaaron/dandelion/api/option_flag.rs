//! Ported from net.azureaaron.dandelion.api.OptionFlag.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::minecraft::Minecraft;

/// java: @FunctionalInterface public interface OptionFlag extends Consumer<Minecraft>
///
/// Any `Fn(&mut Minecraft)` is one. A flag is held as `Arc<dyn OptionFlag>`; Java compares flags by IDENTITY (the
/// `HashSet<OptionFlag>` of `MoulConfigDefinition`), which is `Arc::ptr_eq`.
pub trait OptionFlag {
    /// java: void accept(Minecraft client);
    fn accept(&self, client: &mut Minecraft);
}

impl<F: Fn(&mut Minecraft)> OptionFlag for F {
    fn accept(&self, client: &mut Minecraft) {
        self(client)
    }
}

thread_local! {
    static ASSET_RELOAD_CELL: Arc<dyn OptionFlag> = Arc::new(|client: &mut Minecraft| {
        // java: Minecraft::delayTextureReload - `this.pendingReload = ..`, which the loop's `runTick` drains (as YACL's
        // own ASSET_RELOAD).
        client.pending_reload = true;
    });
    static RELOAD_CHUNKS_CELL: Arc<dyn OptionFlag> = Arc::new(|client: &mut Minecraft| {
        // java: client.levelExtractor.allChanged() - the port's LevelExtractor.allChanged is GameRenderer.all_changed,
        // handed the level (as YACL's own RELOAD_CHUNKS).
        let level = mcrust::mcrust::modding::game::level();
        client.game_renderer.all_changed(level.as_deref());
    });
}

/// java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
#[allow(non_snake_case)]
pub fn ASSET_RELOAD() -> Arc<dyn OptionFlag> {
    ASSET_RELOAD_CELL.with(|f| f.clone())
}

/// java: OptionFlag RELOAD_CHUNKS = client -> client.levelExtractor.allChanged();
#[allow(non_snake_case)]
pub fn RELOAD_CHUNKS() -> Arc<dyn OptionFlag> {
    RELOAD_CHUNKS_CELL.with(|f| f.clone())
}
