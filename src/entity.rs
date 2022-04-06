use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u64,
    pub properties: HashMap<String, Vec<u8>>
}

impl Entity {
    pub fn new() -> Self {
        let id = 0;
        let properties = HashMap::new();

        Self { id, properties }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
