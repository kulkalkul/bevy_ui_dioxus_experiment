use bevy::prelude::Entity;
use dioxus::core::ElementId;

#[derive(Default, Debug)]
pub struct ElementMap {
    entities: Vec<Entity>,
}

impl ElementMap {
    pub fn set(&mut self, id: ElementId, entity: Entity) {
        if id.0 >= self.entities.len() {
            let missing = id.0 - self.entities.len() + 1;

            for _ in 0..missing {
                self.entities.push(Entity::PLACEHOLDER);
            }
        }

        self.entities[id.0] = entity;
    }
    pub fn get(&self, id: ElementId) -> Entity {
        self.entities[id.0]
    }
}