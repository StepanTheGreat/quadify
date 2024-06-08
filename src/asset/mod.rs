use bevy_app::Plugin;
use bevy_asset::{Asset, AssetApp, Assets};
use bevy_asset::{AssetLoader, AssetPlugin as BevyAssetPlugin};
use bevy_reflect::Reflect;
use miniquad::{ShaderMeta, ShaderSource, TextureId};

use crate::prelude::material::Material;
use crate::prelude::Mesh;

pub mod io;
pub use io::*;

// ? I'm using Option here to workaround rendering types not implementing Default trait. If there's a better way
// ? of course - it would be great!

#[derive(Asset, Clone, PartialEq, Reflect)]
pub struct Texture {
	#[reflect(ignore)]
	texture: Option<TextureId>,
}

impl Texture {
	pub fn new(texture: TextureId) -> Self {
		Self { texture: Some(texture) }
	}

	pub fn id(&self) -> TextureId {
		// This shouldn't panic, since textures are supposed to be always Some
		self.texture.unwrap()
	}
}

pub struct AssetPlugin;
impl Plugin for AssetPlugin {
	fn build(&self, app: &mut bevy_app::App) {
		app.add_plugins(BevyAssetPlugin::default())
			.register_asset_reflect::<Mesh>()
			.init_resource::<Assets<Mesh>>()
			.register_asset_reflect::<Texture>()
			.init_resource::<Assets<Texture>>()
			.register_asset_reflect::<Material>()
			.init_resource::<Assets<Material>>();
	}
}
