/// This plugin is an abstraction over macroquad's textures
/// Don't be scared, because it's my first prototype. In the future I'm planning to change it.

use bevy::prelude::*;
use macroquad::prelude::*;
use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Render;
// ? Actually perform drawing operations and wait for the next frame

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreRender;
// ? Prepare objects, update their positions, etc.

#[derive(Component)]
pub struct SpriteTexture(Texture2D);

#[derive(Component)]
pub struct SpriteColor(Color);

#[derive(Component)]
pub struct SpriteFlip {
    x: bool,
    y: bool
}

#[derive(Component)]
pub struct SpriteSize(Vec2);

#[derive(Component)]
pub struct SpriteSrc {
    x: f32, y: f32,
    w: f32, h: f32
}
// ? Source rectangular area from the original texture

#[derive(Component)]
pub struct SpritePivot(Vec2);
// ? Around what point to rotate this texture? (Screen space)

#[derive(Component)]
pub struct SpriteOffset(Vec2);
// ? An optional offset from target

#[derive(Bundle)]
pub struct SpriteBundle {
    transform: Transform,
    texture: SpriteTexture,
    color: SpriteColor,
    flip: SpriteFlip
    // * Additional components can be added manually
}

impl Default for SpriteBundle {
    fn default() -> Self {
        Self {
            transform: Transform::new(0.0, 0.0, 0.0),
            texture: SpriteTexture(Texture2D::empty()),
            color: SpriteColor(WHITE),
            flip: SpriteFlip { x: false, y: false }
        }
    }
}

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(Render)
            .init_schedule(PreRender)
            .add_systems(Render, batch_draw);

        {
            let mut sched_order = app.world.resource_mut::<MainScheduleOrder>();
            sched_order.insert_after(Last, PreRender);
            sched_order.insert_after(PreRender, Render);
        }
    }
}

type BatchComponents<'a> = (
    &'a Transform, &'a SpriteTexture, &'a SpriteColor, &'a SpriteFlip,
    Option<&'a SpriteSize>, Option<&'a SpriteSrc>, Option<&'a SpritePivot>, Option<&'a SpriteOffset>
);
fn batch_draw(
    render_targets: Query<BatchComponents>,
) {
    let mut sprites: Vec<BatchComponents> = render_targets.iter().collect();
    sprites.sort_by(|a, b| a.0.pos.z.partial_cmp(&b.0.pos.z).unwrap());
    for (trns, texture, color, flip, size, src, pivot, offset) in sprites.iter() {
        let offset = if let Some(o) = offset { o.0 } else { Vec2::ZERO };
        draw_texture_ex(
            &texture.0, 
            trns.pos.x+offset.x, 
            trns.pos.y+offset.y, 
            color.0, 
            DrawTextureParams {
                dest_size: if let Some(s) = size { Some(s.0) } else { None },
                source: if let Some(r) = src { Some(Rect::new(r.x, r.y, r.w, r.h)) } else { None },
                rotation: trns.rot,
                flip_x: flip.x,
                flip_y: flip.y,
                pivot: if let Some(p) = pivot { Some(p.0) } else { None }
            }
        )
    }
}