// SOW-013-A: Asset loading system using Bevy AssetServer

pub mod loader;
pub mod registry;

pub use loader::AssetLoaderPlugin;
pub use registry::GameAssets;
