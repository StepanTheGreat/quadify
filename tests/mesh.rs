/// This test should be changed in the future to use more higher-level functionality like `Material`s, to avoid writing repetitive shaders;
/// Thus this test is only for mesh demonstration purposes.
use bevy::prelude::*;
use miniquad::*;
use quadify::prelude::geometry::Vertex;
use quadify::prelude::RenderingBackend;
use quadify::prelude::WindowPlugin;
use quadify::{color, prelude::*};

struct RenderState {
	pipeline: Pipeline,
	bindings: Bindings,
	verts: i32,
}

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			width: 512,
			height: 512,
			title: "Circle Mesh Test".to_owned(),
			high_dpi: false,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Startup, setup_render_state)
		.add_systems(MiniquadDraw, draw_circle)
		.run();
}

fn setup_render_state(world: &mut World) {
	let mut backend = world.get_non_send_resource_mut::<RenderingBackend>().unwrap();
	let mesh = geometry::Mesh::circle(64, color::RED);

	let vbo = backend.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&mesh.vertices[..]));

	let ibo = backend.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&mesh.indices[..]));

	let shader = backend
		.new_shader(
			ShaderSource::Glsl {
				vertex: VERTEX_SHADER,
				fragment: FRAGMENT_SHADER,
			},
			ShaderMeta {
				uniforms: UniformBlockLayout { uniforms: vec![] },
				images: vec![],
			},
		)
		.unwrap();

	let pipeline = backend.new_pipeline(&[BufferLayout::default()], &Vertex::attributes(), shader, PipelineParams::default());

	let bindings = Bindings {
		vertex_buffers: vec![vbo],
		index_buffer: ibo,
		images: vec![],
	};

	world.insert_non_send_resource(RenderState {
		pipeline,
		bindings,
		verts: mesh.indices.len() as i32,
	});
}

fn draw_circle(render_state: NonSend<RenderState>, mut render_ctx: NonSendMut<RenderingBackend>) {
	let (pipeline, bindings, verts) = { (render_state.pipeline.clone(), render_state.bindings.clone(), render_state.verts) };

	render_ctx.begin_default_pass(PassAction::Nothing);
	render_ctx.apply_pipeline(&pipeline);
	render_ctx.apply_bindings(&bindings);
	render_ctx.draw(0, verts, 1);
	render_ctx.end_render_pass();

	// backend.begin_default_pass(PassAction::Clear {
	//     color: Some((0.0, 0.0, 0.0, 1.0)),
	//     depth: None,
	//     stencil: None
	// });
}

const VERTEX_SHADER: &str = r#"#version 100
    precision lowp float;

    attribute vec3 in_pos;
    attribute vec2 in_uv;
    attribute vec4 in_color;

    varying lowp vec4 color;

    void main() {
        color = in_color;
        gl_Position = vec4(in_pos, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"#version 100
precision lowp float;

varying lowp vec4 color;

void main() {
    gl_FragColor = color;
}
"#;
