/// This test should be changed in the future to use more higher-level functionality like `Material`s, to avoid writing repetitive shaders;
/// Thus this test is only for mesh demonstration purposes.
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::mouse::MouseButtonInput;
use bevy_input::prelude::*;

use quadify::{color, prelude::*};
use glam::Mat4;

#[derive(Resource)]
struct MeshHandle {
	mesh: Mesh,
	parts_count: u32,
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
		.add_systems(Update, change_on_click)
		.add_systems(MiniquadDraw, draw_circle)
		.run();
}

fn setup_render_state(mut commands: Commands) {
	commands.insert_resource(MeshHandle {
		mesh: geometry::Mesh::circle(4, color::RED),
		parts_count: 4,
	});
}

fn change_on_click(mut mesh: ResMut<MeshHandle>, mut click: EventReader<MouseButtonInput>) {
	for event in click.read() {
		if event.state.is_pressed() {
			mesh.parts_count = ((mesh.parts_count + 1) % 16).max(4);
			mesh.mesh = geometry::Mesh::circle(mesh.parts_count, color::RED);
		}
	}
}

fn draw_circle(mesh: Res<MeshHandle>, mut render_ctx: NonSendMut<RenderingBackend>) {
	let (verts, inds) = (&mesh.mesh.vertices, &mesh.mesh.indices);
	render_ctx.clear(color::rgba(0, 0, 0, 0));
	render_ctx.texture(None);
	render_ctx.draw_mode(pipeline::DrawMode::Triangles);
	render_ctx.geometry(&verts[..], &inds[..]);
	render_ctx.draw(Mat4::IDENTITY);
}
