#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let cam_offset = Point::new(camera.left_x, camera.top_y);
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let is_player_visible = player_fov.visible_tiles.contains(&pt);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) && is_player_visible | map.revealed_tiles[idx] {
                let tint = if is_player_visible { WHITE } else { DARK_GRAY };
                let glyph = theme.tile_to_render(map.tiles[idx]);
                draw_batch.set(pt - cam_offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error");
}
