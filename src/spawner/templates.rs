#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;
use std::fs::File;

use legion::systems::CommandBuffer;
use legion::World;
use ron::de::from_reader;
use serde::Deserialize;

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub name: String,
    pub frequency: i32,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Fail opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new(); // (13)
        self.entities
            .iter() // (14)
            .filter(|e| e.levels.contains(&level)) // (15)
            .for_each(|t| {
                for _ in 0..t.frequency {
                    // (16)
                    available_entities.push(t);
                }
            });

        let mut commands = CommandBuffer::new(ecs); // (17)
        spawn_points.iter().for_each(|pt| {
            // (18)
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                // (19)
                self.spawn_entity(pt, entity, &mut commands); // (20)
            }
        });
        commands.flush(ecs);
    }

    fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut CommandBuffer) {
        let entity = commands.push((
            // (21)
            pt.clone(), // (22)
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph), // (23)
            },
            Name(template.name.clone()), // (24)
        ));
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }
        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                });
        }
    }
}
