#![warn(clippy::all, clippy::pedantic)]
use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let cam_offset = Point::new(camera.left_x, camera.top_y);
    let mut draw_batch = DrawBatch::new();
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();
    draw_batch.target(1);
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - cam_offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch Error");
}
