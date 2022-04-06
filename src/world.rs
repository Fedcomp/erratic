use crate::Entity;

pub struct World {
    last_entity_id: u64,
    entities: Vec<Entity>
}

impl World {
    pub fn new() -> Self {
        let last_entity_id = 0;
        let entities = Vec::new();
        Self { last_entity_id, entities }
    }

    pub fn insert(&mut self, mut entity: Entity) {
        entity.id = self.last_entity_id;
        self.entities.push(entity);
        self.last_entity_id += 1;
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

#[cfg(test)]
mod tests {
    use crate::Entity;
    use super::World;

    #[test]
    fn test_insert() {
        let mut world = World::new();
        let entity = Entity::new();

        world.insert(entity.clone());
        world.insert(entity.clone());

        assert_eq!(world.entities.len(), 2);
        assert_eq!(world.entities[0].id(), 0);
        assert_eq!(world.entities[1].id(), 1);
    }
}
