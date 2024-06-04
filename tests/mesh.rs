/// This test should be changed in the future to use more higher-level functionality like `Material`s, to avoid writing repetitive shaders;
/// Thus this test is only for mesh demonstration purposes.
use bevy_app::prelude::*;
use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_input::mouse::MouseButtonInput;
use glam::Mat4;
use quadify::prelude::geometry::{Mesh, MeshBuilder};
use quadify::prelude::RenderingBackend;
use quadify::prelude::WindowPlugin;
use quadify::{color, prelude::*};

#[derive(Resource)]
struct MeshHandle {
	mesh: Handle<Mesh>,
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

fn setup_render_state(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
	commands.insert_resource(MeshHandle {
		mesh: meshes.add(MeshBuilder::new().as_circle(0.2).circle_points(3).build()),
		parts_count: 3,
	});
}

fn change_on_click(mut mesh: ResMut<MeshHandle>, mut click: EventReader<MouseButtonInput>, mut meshes: ResMut<Assets<Mesh>>) {
	for event in click.read() {
		if event.state.is_pressed() {
			mesh.parts_count = ((mesh.parts_count + 1) % 64).max(4);
			meshes.insert(&mesh.mesh, MeshBuilder::new().as_circle(0.2).circle_points(mesh.parts_count).build());
		}
	}
}

fn draw_circle(mesh: Res<MeshHandle>, mut render_ctx: NonSendMut<RenderingBackend>, meshes: Res<Assets<Mesh>>, clear_colour: Res<ClearColor>) {
	if let Some(mesh) = meshes.get(&mesh.mesh) {
		let (verts, inds) = (&mesh.vertices, &mesh.indices);
		render_ctx.clear(clear_colour.0);
		render_ctx.texture(None);
		render_ctx.draw_mode(pipeline::DrawMode::Triangles);
		render_ctx.geometry(&verts[..], &inds[..]);
		render_ctx.draw(Mat4::IDENTITY);
	}
}
