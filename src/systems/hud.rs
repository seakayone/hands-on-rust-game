#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Point)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );


    let player_entity = <(Entity, &Player)>::query().iter(ecs).find_map(|(entity, _player)| Some(*entity)).unwrap();

    let mut carried = <(&Item, &Carried, &Name)>::query();
    draw_batch.print(Point::new(3, 2), "K : Item");
    let mut y = 3;
    carried.iter(ecs)
        .filter(|(_, carried, _)| carried.0 == player_entity)
        .for_each(|(_, _, name)| {
            let key_label = if y - 2 < 10 { (y - 2).to_string() } else { " ".to_string() };
            draw_batch.print(Point::new(3, y), format!("{} : {}", key_label, &name.0));
            y += 1;
        });

    let (player_pos, player) = <(&Point, &Player)>::query().iter(ecs).nth(0).unwrap();
    draw_batch.print_right(Point::new(SCREEN_WIDTH * 2, 1), format!("Level: {} | Pos: x {}, y {}", player.map_level + 1, player_pos.x, player_pos.y));

    draw_batch.submit(10000).expect("Batch error");
}
