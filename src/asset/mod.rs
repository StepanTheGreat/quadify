use std::sync::OnceLock;

use bevy_app::Plugin;
use bevy_asset::AssetPlugin as BevyAssetPlugin;
use bevy_asset::{Asset, AssetApp, Assets};
use bevy_reflect::Reflect;
use miniquad::TextureId;

use crate::prelude::material::Material;
use crate::prelude::Mesh;

#[derive(Asset, Clone, PartialEq, Reflect)]
pub struct Texture {
	#[reflect(ignore)]
	texture: OnceLock<TextureId>,
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
