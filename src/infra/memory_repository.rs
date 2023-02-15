use std::collections::hash_map::Entry;
use std::collections::HashMap;
use domain::time_record::TimeRecord;
use crate::domain;
use crate::usecase::simple_usecase::Repository;
use crate::usecase::usecase;
use crate::usecase::usecase::MyError;

pub struct MemoryRepository {
    map: HashMap<String, TimeRecord>,
}

impl MemoryRepository {
    pub fn new() -> Box<dyn Repository> {
        Box::new(MemoryRepository {
            map: HashMap::new(),
        })
    }
}

impl Repository for MemoryRepository {
    fn create(&mut self, record: TimeRecord) -> usecase::Result<()> {
        match self.map.entry(record.id().to_string()) {
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
                Err(Box::new(MyError {}))
            }
            Some(_) => {
                Ok(())
            }
        }
    }

    fn list(&mut self) -> usecase::Result<Vec<TimeRecord>> {
        Ok(self.map.values().cloned().collect::<Vec<TimeRecord>>())
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;
    use crate::domain;
    use crate::infra::memory_repository::MemoryRepository;

    #[test]
    fn internal() {
        let mut repo = MemoryRepository::new();
        let time = Local::now();
        repo.create(domain::time_record::TimeRecord::new_with_id("12".to_string(), time, time, "memo".to_string())).unwrap();

        let ret = repo.list().unwrap();
        assert_eq!(ret.len(), 1);
        assert_eq!(ret[0].start(), time);

        repo.delete("12".to_string()).unwrap();

        let err = repo.delete("12".to_string());
        assert_eq!(err.is_err(), true);
    }
}

