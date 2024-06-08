use std::marker::PhantomData;

use bevy_ecs::system::{NonSendMut, SystemParam};
use miniquad::ShaderSource;
use miniquad::TextureId;

use crate::io::load_file_sync;
use crate::prelude::material::{Material, MaterialParams};
use crate::prelude::RenderingBackend;

use super::Texture;

/// Loads a texture and automatically pushes it to GPU.
fn load_texture(path: impl Into<&'static str>, format: Option<image::ImageFormat>, backend: &mut RenderingBackend) -> Option<TextureId> {
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
	let texture = backend.new_texture_from_rgba8(img.width() as u16, img.height() as u16, &img.into_raw());
	Some(texture)
}

struct Null;

/// A syntatic sugar for loading files, textures and materials on the same thread. This simplifies marking systems as exclusive, and
/// pushing the data like textures and materials directly to GPU.
///
/// * Note: Returned assets aren't added to assets automatically *
#[derive(SystemParam)]
pub struct AssetIO<'w, 's> {
	backend: NonSendMut<'w, RenderingBackend>,
	_m: PhantomData<&'s Null>, // I'm really desperate on this one
}

impl<'w, 's> AssetIO<'w, 's> {
	pub fn load_bytes(&self, path: impl Into<&'static str>) -> Result<std::vec::Vec<u8>, miniquad::fs::Error> {
		load_file_sync(path)
	}

	pub fn load_texture(&mut self, path: impl Into<&'static str>, format: Option<image::ImageFormat>) -> Option<Texture> {
		match load_texture(path, format, &mut self.backend) {
			Some(texture) => Some(Texture::new(texture)),
			None => None,
		}
	}

	pub fn load_material(&mut self, src: ShaderSource<'static>, params: MaterialParams) -> Option<Material> {
		let material = self.backend.request_material(src, params);

		match material {
			Ok(m) => Some(m),
			Err(err) => {
				#[cfg(feature = "log")]
				bevy_log::error!("{:?}", err);
				None
			}
		}
	}
}
