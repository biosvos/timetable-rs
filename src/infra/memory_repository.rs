use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::domain;
use crate::usecase::simple_usecase::Repository;
use crate::usecase::usecase;
use crate::usecase::usecase::{MyError, TimeRecordWithID};

pub struct MemoryRepository {
    map: HashMap<String, TimeRecordWithID>,
}

impl MemoryRepository {
    pub fn new() -> MemoryRepository {
        MemoryRepository {
            map: HashMap::new(),
        }
    }
}

impl Repository for MemoryRepository {
    fn create(&mut self, record: domain::time_record::TimeRecord) -> usecase::Result<()> {
        match self.map.entry(record.id.to_string()) {
            Entry::Occupied(_) => {
                Err(Box::new(MyError {}))
            }
            Entry::Vacant(entry) => {
                entry.insert(record);
                Ok(())
            }
        }
    }

    fn delete(&mut self, id: String) -> usecase::Result<()> {
        match self.map.remove(&id) {
            None => {
                Ok(())
            }
            Some(_) => {
                Err(Box::new(MyError {}))
            }
        }
    }

    fn list(&mut self) -> usecase::Result<Vec<TimeRecordWithID>> {
        Ok(self.map.values().cloned().collect::<Vec<TimeRecordWithID>>())
    }
}

