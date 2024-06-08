use std::marker::PhantomData;

use bevy_asset::{Assets, Handle};
use bevy_ecs::system::{NonSendMut, ResMut, SystemParam};
use miniquad::{fs::load_file, ShaderSource};

use crate::prelude::{
	material::{Material, MaterialParams},
	RenderingBackend,
};

use super::Texture;

/// Loads a file syncronously, and returns a vector of bytes on success. Uses HTTPs on web.
///
/// Make sure to use it only in exclusive systems.
pub fn load_file_sync(path: &str) -> Result<Vec<u8>, miniquad::fs::Error> {
	let (tx, rx) = oneshot::channel();
	load_file(path, move |data| {
		tx.send(data).unwrap();
	});

	// ? Shouldn't panic
	rx.recv().unwrap()
}

#[derive(SystemParam)]
pub struct QuadAssetLoader<'w, Marker: 'static> {
	backend: NonSendMut<'w, RenderingBackend>,
	materials: ResMut<'w, Assets<Material>>,
	textures: ResMut<'w, Assets<Texture>>,
	_m: PhantomData<Marker>,
}

impl<'w, Marker: 'static> QuadAssetLoader<'w, Marker> {
	pub fn load_texture(&mut self, path: impl Into<&'static str>, format: Option<image::ImageFormat>) -> Option<Handle<Texture>> {
		let bytes = match load_file_sync(path.into()) {
			Ok(bytes) => bytes,
			Err(err) => {
				#[cfg(feature = "log")]
				bevy_log::error!("{:?}", err);
				return None;
			}
		};

		let img = if let Some(fmt) = format {
			match image::load_from_memory_with_format(&bytes, fmt) {
				Ok(img) => img.to_rgba8(),
				Err(err) => {
					#[cfg(feature = "log")]
					bevy_log::error!("{:?}", err);
					return None;
				}
			}
		} else {
			match image::load_from_memory(&bytes) {
				Ok(img) => img.to_rgba8(),
				Err(err) => {
					#[cfg(feature = "log")]
					bevy_log::error!("{:?}", err);
					return None;
				}
			}
		};
		let texture_id = self.backend.new_texture_from_rgba8(img.width() as u16, img.height() as u16, &img.into_raw());
		Some(self.textures.add(Texture::new(texture_id)))
	}

	pub fn load_material(&mut self, src: ShaderSource<'static>, params: MaterialParams) -> Option<Handle<Material>> {
		let material = self.backend.request_material(src, params);

		match material {
			Ok(m) => Some(self.materials.add(m)),
			Err(err) => {
				#[cfg(feature = "log")]
				bevy_log::error!("{:?}", err);
				None
			}
		}
	}
}
