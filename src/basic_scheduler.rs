use crate::System;
use crate::{SchedulerContext, SchedulerContextChange};

pub struct BasicScheduler {
    execution_order: Vec<System>
}

impl BasicScheduler {
    /// Create new scheduler instance
    pub fn new() -> Self {
        Self {
            execution_order: Vec::new()
        }
    }

    /// Append [System] to the end of scheduler execution order
    pub fn add_system(&mut self, system: System) {
        self.execution_order.push(system)
    }

    pub fn get_execution_order(&mut self) -> &[System] {
        &self.execution_order
    }

    /// One time run of all scheduled systems
    pub fn tick(&mut self) {
        let mut changes: Vec<SchedulerContextChange> = Vec::new();

        for system in self.execution_order.iter() {
            let mut system_context = SchedulerContext::new();
            system.call(&mut system_context);
            changes.append(&mut system_context.into_changes());
        }

        for change in changes.into_iter() {
            match change {
                SchedulerContextChange::AddSystem(system) => {
                    self.execution_order.push(system);
                }
            }
        }
    }
}
