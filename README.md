# Quadify

## Quadify is a bevy plugin with a minimal set of bevy's features. It uses macroquad for windowing/graphics/sound

If an enormous bevy's dependency tree is too much for your game - you may want to try a simpler solution. This
plugin provides you with macroquad windowing/rendering/sound, while also trying to use existing, known to you bevy's API.
If you're doing simple web-games with 2D graphics - this might be suitable for you.

*(Note: I'm not macroquad nor bevy developer, so please check their respected projects first; I'm just combining these two in a simple plugin.)*

## Planned features:

| feature name | description                   | reference (bevy)               |is required |status|
| ---          | ---                           | ---                            | ---        | ---  |
| quad_window  | Window management and events  | bevy_window                    | ❗        | ⚒️   |
| quad_input   | Input types                   | bevy_input                     | ❗        | ⚒️   |
| parallelism  | Support for parallelism       | None                           | ❗        | ❌   |
| quad_render  | Basic rendering abstractions  | bevy_render/bevy_core_pipeline | ❔        | ❌   |
| quad_asset   | Really basic asset management | bevy_asset                     | ❔        | ❌   |
| quad_sprite  | Sprite rendering              | bevy_sprite                    | ❔        | ❌   |
| quad_text    | Text rendering                | bevy_text                      | ❔        | ❌   |
| quad_ui      | GUI from macroquad            | bevy_ui                        | ❔        | ❌   |
| quad_audio   | Audio functionality           | bevy_audio                     | ❔        | ❌   |

*This list was composed on my personal needs, if the project gains attention I'll maybe try to add other functionality as well (But with no bloat)*

## Platform support

I believe the platform support will be the same as macroquad's, so users might have to build their games using macroquad's instructions for each platform.

## Licensing

I'm leaving the same MIT and APACHE licenses from both projects for you to choose.
