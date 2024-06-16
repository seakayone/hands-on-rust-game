#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.range(0, 4) {
        0 => goblin(),
        1 => orc(),
        2 => ogre(),
        _ => ettin(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        ChasingPlayer,
        FieldOfView::new(6),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
fn ettin() -> (i32, String, FontCharType) {
    (3, "Ettin".to_string(), to_cp437('E'))
}
fn ogre() -> (i32, String, FontCharType) {
    (4, "Ogre".to_string(), to_cp437('O'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        ProvidesHealing(6),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
    ));
}
pub fn spawn_dungeon_map(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        ProvidesDungeonMap {},
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Scroll of Dungeon Mapping".to_string()),
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_dungeon_map(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    };
}
