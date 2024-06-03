#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        };
        for y in 0..SCREEN_HEIGHT {
            map.tiles[map_idx(0, y)] = TileType::Wall;
            map.tiles[map_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
        }
        for x in 0..SCREEN_WIDTH {
            map.tiles[map_idx(x, 0)] = TileType::Wall;
            map.tiles[map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
        }
        map
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Wall => {
                        ctx.set(x, y, WHITE, BLACK, to_cp437('#'));
                    }
                    TileType::Floor => {
                        ctx.set(x, y, WHITE, BLACK, to_cp437('.'));
                    }
                }
            }
        }
    }
}
