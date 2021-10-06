type SystemFunction = Box<dyn Fn() -> ()>;

pub struct System {
    f: SystemFunction
}

impl System {
    pub fn new(f: SystemFunction) -> System {
        System { f }
    }

    pub fn call(&self) {
        (self.f)();
    }
}
