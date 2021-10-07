#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u64
}

impl Entity {
    pub fn new() -> Self {
        let id = 0;
        Self { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
