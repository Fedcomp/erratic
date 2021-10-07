use crate::System;

pub enum SchedulerContextChange {
    AddSystem(System)
}

pub struct SchedulerContext {
    changes: Vec<SchedulerContextChange>
}

impl SchedulerContext {
    pub fn new() -> Self {
        SchedulerContext {
            changes: Vec::new()
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.changes.push(SchedulerContextChange::AddSystem(system));
    }

    pub fn into_changes(self) -> Vec<SchedulerContextChange> {
        self.changes
    }
}
