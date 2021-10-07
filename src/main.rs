use erratic::{System, BasicScheduler, SchedulerContext};

fn greetings(context: &mut SchedulerContext) {
    println!("Greetings!");
    let another_greeting = System::new(Box::new(greetings));
    context.add_system(another_greeting);
}

fn bye(_context: &mut SchedulerContext) {
    println!("Bye!");
}

fn main() {
    let greetings_system = System::new(Box::new(greetings));
    let bye_system = System::new(Box::new(bye));

    let mut scheduler = BasicScheduler::new();

    scheduler.add_system(greetings_system);
    scheduler.add_system(bye_system);
    scheduler.tick();
    scheduler.tick();
}
