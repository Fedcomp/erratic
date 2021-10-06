use erratic::{System, BasicScheduler};

fn main() {
    let greetings_system = System::new(Box::new(|| println!("Greetings!")) );

    let bye_system = System::new(Box::new(|| println!("Bye!")) );
    let mut scheduler = BasicScheduler::new();

    scheduler.add_system(greetings_system);
    scheduler.add_system(bye_system);

    scheduler.tick();
}
