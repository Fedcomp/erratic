use crate::SchedulerContext;

type SystemFunction = Box<dyn Fn(&mut SchedulerContext) -> ()>;

pub struct System {
    f: SystemFunction
}

impl System {
    pub fn new(f: SystemFunction) -> System {
        System { f }
    }

    pub fn call(&self, context: &mut SchedulerContext) {
        (self.f)(context);
    }
}
