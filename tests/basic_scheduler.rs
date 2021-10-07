use erratic::{System, BasicScheduler, SchedulerContext};

fn hydra(context: &mut SchedulerContext) {
    let another_head = System::new(Box::new(hydra));
    context.add_system(another_head);
}

#[test]
fn test_system_adds_system() {
    let mut scheduler = BasicScheduler::new();

    let hydra_system = System::new(Box::new(hydra));
    scheduler.add_system(hydra_system);
    assert_eq!(scheduler.get_execution_order().len(), 1);
    scheduler.tick();
    assert_eq!(scheduler.get_execution_order().len(), 2);
}
