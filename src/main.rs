use std::error::Error;
use crate::infra::memory_repository::MemoryRepository;
use crate::infra::uuid_id_generator::UuidIdGenerator;
use crate::usecase::simple_usecase::SimpleUsecase;
use crate::usecase::usecase::{TimeRecord, Usecase};

mod domain;
mod usecase;
mod infra;

fn main() -> Result<(), Box<dyn Error>> {
    let mut usecase = SimpleUsecase::new(Box::new(MemoryRepository::new()), Box::new(UuidIdGenerator::new()));
    match usecase.create_time_record(TimeRecord {
        start: "2023-02-10 21:24:50".to_string(),
        end: "2023-02-10 21:24:50".to_string(),
        memo: "".to_string(),
    }) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }

    for record in usecase.list_time_records()? {
        println!("{} {} ~ {} {}", record.id, record.start, record.end, record.memo);
    }
    Ok(())
}
