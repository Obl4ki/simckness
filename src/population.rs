use itertools::Itertools;

use crate::constants::N_ENTITIES;
use crate::entities::entity::Entity;

#[derive(Debug, Clone)]
pub struct Population {
    pub entities: Vec<Entity>,
}

impl Population {
    pub fn new() -> Self {
        Self {
            entities: (0..N_ENTITIES)
                .map(|_| Entity::new_random().unwrap())
                .collect(),
        }
    }

    pub fn advance(&self) -> Self {
        let next_turn_entities = self
            .entities
            .clone()
            .into_iter()
            .map(|mut entity| {
                entity.make_move();
                entity.age.0 += 1;
                entity
            })
            .map(|mut entity| {
                self.entities.iter().for_each(|other_entity| {
                    entity.check_contact(other_entity);
                });
                entity
            })
            .map(|mut entity| {
                entity.apply_health_effect();
                entity.advance_health_by_day();
                entity
            })
            .flat_map(|mut entity| {
                let mut children = entity.current_children.drain(..).collect_vec();
                children.push(entity);
                children
            })
            .filter(|entity| !entity.should_die())
            .collect();

        Self {
            entities: next_turn_entities,
        }
    }
}
