use crate::System;

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

    /// One time run of all scheduled systems
    pub fn tick(&mut self) {
        for system in self.execution_order.iter() {
            system.call();
        }
    }
}
