#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => pick_up_item(ecs, commands),
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            VirtualKeyCode::Key0 => use_item(9, ecs, commands),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        };
        *turn_state = TurnState::PlayerTurn;
    }

    fn pick_up_item(ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, player_pos) = players.iter(ecs)
            .find_map(|(entity, pos)| Some((entity, pos)))
            .unwrap();
        let mut items = <(Entity, &Item, &Point)>::query();
        items.iter(ecs)
            .for_each(|(e, i, p)| {
                if player_pos == p {
                    commands.remove_component::<Point>(*e);
                    commands.add_component(*e, Carried(*player));
                };
            });
        Point::zero()
    }

    fn use_item(n: usize, ecs: &SubWorld, commands: &mut CommandBuffer) -> Point {
        println!("Item {}", n);
        let player_entity = <(Entity, &Player)>::query().iter(ecs)
            .find_map(|(entity, _player)| Some(*entity)).unwrap();
        let item_entity = <(Entity, &Item, &Carried)>::query().iter(ecs)
            .filter(|(_, _, carried)| carried.0 == player_entity)
            .enumerate()
            .filter(|(item_count, (_, _, _))| *item_count == n)
            .find_map(|(_, (item_entity, _, _))| Some(*item_entity));
        if let Some(item_entity) = item_entity {
            println!("Using {}", n);
            commands
                .push(((), ActivateItem {
                    used_by: player_entity,
                    item: item_entity,
                }));
        }
        Point::zero()
    }
}
