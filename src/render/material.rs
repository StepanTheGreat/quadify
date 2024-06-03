use miniquad::*;
use super::Rgba;
use bevy_asset::Asset;
use bevy_reflect::Reflect;

use crate::render::GlPipeline;

/// Material instance loaded on GPU.
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
pub struct DefaultMateralParams {
    color: Rgba,
    texture: Option<TextureId>,

}