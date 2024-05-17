use miniquad::*;
use std::collections::BTreeMap;

const fn as_bytes<T, const SIZE: usize>() -> [u8; SIZE] {
	debug_assert!(std::mem::size_of::<T>() == SIZE);
	unsafe { std::mem::transmute([0u8; SIZE]) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlPipeline(usize);

#[derive(Clone, Debug)]
struct Uniform {
	name: String,
	uniform_type: UniformType,
	byte_offset: usize,
}

#[derive(Clone)]
struct PipelineExt {
	pipeline: miniquad::Pipeline,
	wants_screen_texture: bool,
	uniforms: Vec<Uniform>,
	uniforms_data: Vec<u8>,
	textures: Vec<String>,
	textures_data: BTreeMap<String, TextureId>,
}

#[repr(transparent)]
pub struct PipelineStorage {
	pipelines: [Option<PipelineExt>; Self::MAX_PIPELINES],
}

impl PipelineStorage {
	const MAX_PIPELINES: usize = 32;
	const TRIANGLES_PIPELINE: GlPipeline = GlPipeline(0);
	const LINES_PIPELINE: GlPipeline = GlPipeline(1);
	const TRIANGLES_DEPTH_PIPELINE: GlPipeline = GlPipeline(2);
	const LINES_DEPTH_PIPELINE: GlPipeline = GlPipeline(3);

	pub(crate) fn new(ctx: &mut dyn RenderingBackend) -> PipelineStorage {
		let source = match ctx.info().backend {
			Backend::OpenGl => ShaderSource::Glsl {
				vertex: shader::VERTEX,
				fragment: shader::FRAGMENT,
			},
			Backend::Metal => ShaderSource::Msl { program: shader::METAL },
		};

		let shader = ctx.new_shader(source, shader::meta()).unwrap();
		let params = PipelineParams {
			color_blend: Some(BlendState::new(
				Equation::Add,
				BlendFactor::Value(BlendValue::SourceAlpha),
				BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
			)),
			..Default::default()
		};

		let mut storage = PipelineStorage { pipelines: Default::default() };

		let triangles_pipeline = storage.make_pipeline(
			ctx,
			shader,
			PipelineParams {
				primitive_type: PrimitiveType::Triangles,
				..params
			},
			false,
			vec![],
			vec![],
		);
		assert_eq!(triangles_pipeline, Self::TRIANGLES_PIPELINE);

		let lines_pipeline = storage.make_pipeline(
			ctx,
			shader,
			PipelineParams {
				primitive_type: PrimitiveType::Lines,
				..params
			},
			false,
			vec![],
			vec![],
		);
		assert_eq!(lines_pipeline, Self::LINES_PIPELINE);

		let triangles_depth_pipeline = storage.make_pipeline(
			ctx,
			shader,
			PipelineParams {
				depth_write: true,
				depth_test: Comparison::LessOrEqual,
				primitive_type: PrimitiveType::Triangles,
				..params
			},
			false,
			vec![],
			vec![],
		);
		assert_eq!(triangles_depth_pipeline, Self::TRIANGLES_DEPTH_PIPELINE);

		let lines_depth_pipeline = storage.make_pipeline(
			ctx,
			shader,
			PipelineParams {
				depth_write: true,
				depth_test: Comparison::LessOrEqual,
				primitive_type: PrimitiveType::Lines,
				..params
			},
			false,
			vec![],
			vec![],
		);
		assert_eq!(lines_depth_pipeline, Self::LINES_DEPTH_PIPELINE);

		storage
	}

	fn make_pipeline(
		&mut self,
		ctx: &mut dyn RenderingBackend,
		shader: ShaderId,
		params: PipelineParams,
		wants_screen_texture: bool,
		mut uniforms: Vec<(String, UniformType)>,
		textures: Vec<String>,
	) -> GlPipeline {
		let pipeline = ctx.new_pipeline(
			&[BufferLayout::default()],
			&[
				VertexAttribute::new("position", VertexFormat::Float3),
				VertexAttribute::new("texcoord", VertexFormat::Float2),
				VertexAttribute::new("color0", VertexFormat::Byte4),
			],
			shader,
			params,
		);

		let id = self.pipelines.iter().position(|p| p.is_none()).expect("Pipelines amount exceeded");
		let mut max_offset = 0;

		for (name, kind) in shader::uniforms().into_iter().rev() {
			uniforms.insert(0, (name.to_owned(), kind));
		}

		let uniforms = uniforms
			.iter()
			.scan(0, |offset, uniform| {
				let uniform_byte_size = uniform.1.size();
				let uniform = Uniform {
					name: uniform.0.clone(),
					uniform_type: uniform.1,
					byte_offset: *offset,
				};
				*offset += uniform_byte_size;
				max_offset = *offset;

				Some(uniform)
			})
			.collect();

		self.pipelines[id] = Some(PipelineExt {
			pipeline,
			wants_screen_texture,
			uniforms,
			uniforms_data: vec![0; max_offset],
			textures,
			textures_data: BTreeMap::new(),
		});

		GlPipeline(id)
	}
}

mod shader {
	use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

	pub const VERTEX: &str = r#"#version 100
	attribute vec3 position;
	attribute vec2 texcoord;
	attribute vec4 color0;

	varying lowp vec2 uv;
	varying lowp vec4 color;

	uniform mat4 Model;
	uniform mat4 Projection;

	void main() {
		 gl_Position = Projection * Model * vec4(position, 1);
		 color = color0 / 255.0;
		 uv = texcoord;
	}"#;

	pub const FRAGMENT: &str = r#"#version 100
	varying lowp vec4 color;
	varying lowp vec2 uv;

	uniform sampler2D Texture;

	void main() {
		 gl_FragColor = color * texture2D(Texture, uv) ;
	}"#;

	pub const METAL: &str = r#"
#include <metal_stdlib>
	using namespace metal;

	struct Uniforms
	{
		 float4x4 Model;
		 float4x4 Projection;
	};

	struct Vertex
	{
		 float3 position    [[attribute(0)]];
		 float2 texcoord    [[attribute(1)]];
		 float4 color0      [[attribute(2)]];
	};

	struct RasterizerData
	{
		 float4 position [[position]];
		 float4 color [[user(locn0)]];
		 float2 uv [[user(locn1)]];
	};

	vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& uniforms [[buffer(0)]])
	{
		 RasterizerData out;

		 out.position = uniforms.Model * uniforms.Projection * float4(v.position, 1);
		 out.color = v.color0 / 255.0;
		 out.uv = v.texcoord;

		 return out;
	}

	fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> tex [[texture(0)]], sampler texSmplr [[sampler(0)]])
	{
		 return in.color * tex.sample(texSmplr, in.uv);
	}
	"#;
	pub fn uniforms() -> Vec<(&'static str, UniformType)> {
		vec![("Projection", UniformType::Mat4), ("Model", UniformType::Mat4), ("_Time", UniformType::Float4)]
	}

	pub fn meta() -> ShaderMeta {
		ShaderMeta {
			images: vec!["Texture".to_string(), "_ScreenTexture".to_string()],
			uniforms: UniformBlockLayout {
				uniforms: uniforms().into_iter().map(|(name, kind)| UniformDesc::new(name, kind)).collect(),
			},
		}
	}
}
