use crate::{Entity, System, World};

pub enum SchedulerContextChange {
    AddSystem(System)
}

pub struct SchedulerContext<'a> {
    world: &'a mut World,
    changes: Vec<SchedulerContextChange>
}

impl <'a> SchedulerContext<'a> {
    pub fn new(world: &'a mut World) -> Self {
        let changes = Vec::new();
        SchedulerContext { world, changes }
    }

    pub fn add_system(&mut self, system: System) {
        self.changes.push(SchedulerContextChange::AddSystem(system));
    }

    pub fn into_changes(self) -> Vec<SchedulerContextChange> {
        self.changes
    }

    pub fn get_entities(&self) -> &[Entity] {
        self.world.entities()
    }
}
