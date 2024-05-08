use image::DynamicImage;
use miniquad::{conf::Icon, fs};
use std::io;

/// Ergonomic interface to [`Icon`].
#[repr(transparent)]
pub struct WindowIcon(image::DynamicImage);

impl WindowIcon {
	/// Load an icon from filesystem on Desktop and Fetch on Web
	pub fn from_file<A: AsRef<str>, F: Fn(Result<WindowIcon, fs::Error>) + 'static>(path: A, callback: F, format: Option<image::ImageFormat>) {
		fs::load_file(path.as_ref(), move |res| match res {
			// File was unable to load
			Err(e) => callback(Err(e)),
			Ok(data) => callback(WindowIcon::from_bytes(&data, format)),
		});
	}

	/// Exactly the same as [`from_file`](WindowIcon::from_file) but async
	pub async fn from_file_async<A: AsRef<str>>(path: A, format: Option<image::ImageFormat>) -> Result<WindowIcon, fs::Error> {
		let data = crate::io::load_file(path.as_ref()).await?;
		WindowIcon::from_bytes(&data, format)
	}

	/// Load an icon from a byte slice
	pub fn from_bytes(data: &[u8], format: Option<image::ImageFormat>) -> Result<WindowIcon, fs::Error> {
		let data = io::Cursor::new(data);

		let reader = match format {
			Some(f) => {
				let mut reader = image::io::Reader::new(data);
				reader.set_format(f);
				reader
			}
			None => image::io::Reader::new(data).with_guessed_format().map_err(fs::Error::IOError)?,
		};

		let img = reader.decode().map_err(|err| {
			log::error!("Failed to decode image: {:?}", err);
			fs::Error::IOSAssetNoData
		})?;
		Ok(WindowIcon(img))
	}
}

/// Given an image, downsample it to a smaller size and decode it into a byte array.
fn downsample<const T: usize, const W: usize>(img: &DynamicImage) -> Option<[u8; T]> {
	let height = T / W;
	let thumbnail = img.thumbnail(W as _, height as _);

	match thumbnail {
		image::DynamicImage::ImageRgba8(rgba8) => {
			let bytes = rgba8.into_raw();
			let mut result: [u8; T] = [0; T];
			result.copy_from_slice(&bytes);
			Some(result)
		}
		_ => None,
	}
}

impl TryFrom<&WindowIcon> for Icon {
	type Error = &'static str;

	fn try_from(value: &WindowIcon) -> Result<Self, Self::Error> {
		static INVALID_IMAGE_FORMAT: &str = "Invalid image format, expected RGBA8 image";
		let img = &value.0;

		Ok(Icon {
			small: downsample::<1024, 16>(img).ok_or(INVALID_IMAGE_FORMAT)?,
			medium: downsample::<4096, 32>(img).ok_or(INVALID_IMAGE_FORMAT)?,
			big: downsample::<16384, 64>(img).ok_or(INVALID_IMAGE_FORMAT)?,
		})
	}
}
