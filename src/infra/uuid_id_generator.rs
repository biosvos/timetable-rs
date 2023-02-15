use crate::usecase::simple_usecase::IdGenerator;

pub struct UuidIdGenerator;

impl UuidIdGenerator {
    pub fn new() -> Box<dyn IdGenerator> {
        Box::new(UuidIdGenerator {})
    }
}


impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

