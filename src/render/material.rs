use super::{RenderingBackend, Rgba};
use bevy_asset::Asset;
use bevy_reflect::Reflect;
use miniquad::*;

use crate::render::GlPipeline;

/// Material instance loaded on GPU. 
/// 
/// Please use the [`RenderingBackend`] non-send resource to modify its data:
/// - [`material_set_uniform`](RenderingBackend::material_set_uniform)
/// - [`material_set_texture`](RenderingBackend::material_set_texture)
#[derive(Asset, Clone, PartialEq, Reflect)]
pub struct Material {
	pub(crate) pipeline: GlPipeline,
}

/// A struct used for requesting to create new materials
#[derive(Clone, Debug)]
pub struct MaterialParams {
	/// miniquad pipeline configuration for this material.
	/// Things like blending, culling, depth dest
	pub pipeline_params: PipelineParams,

	/// List of custom uniforms used in this material
	pub uniforms: Vec<(String, UniformType)>,

	/// List of textures used in this material
	pub textures: Vec<String>,
}

#[derive(Debug)]
pub struct DefaultMaterailParams {
	color: Rgba,
	texture: Option<TextureId>,
}
