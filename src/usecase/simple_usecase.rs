use chrono::{DateTime, Local, TimeZone};
use crate::domain;
use crate::usecase::usecase::{TimeRecord, TimeRecordWithID, Usecase, Result};

pub trait Repository {
    fn create(&mut self, record: domain::time_record::TimeRecord) -> Result<()>;
    fn delete(&mut self, id: String) -> Result<()>;
    fn list(&mut self) -> Result<Vec<domain::time_record::TimeRecord>>;
}

pub trait IdGenerator {
    fn generate(&self) -> String;
}

pub struct SimpleUsecase {
    repository: Box<dyn Repository>,
    id_generator: Box<dyn IdGenerator>,
}

impl SimpleUsecase {
    pub fn new(repository: Box<dyn Repository>, id_generator: Box<dyn IdGenerator>) -> SimpleUsecase {
        SimpleUsecase {
            repository,
            id_generator,
        }
    }

    fn parse_time(time: String) -> Result<DateTime<Local>> {
        let datetime = Local.datetime_from_str(&time, "%Y-%m-%d %H:%M:%S")?;
        Ok(DateTime::from(datetime))
    }
}

impl Usecase for SimpleUsecase {
    fn create_time_record(&mut self, record: TimeRecord) -> Result<()> {
        let start = SimpleUsecase::parse_time(record.start)?;
        let end = SimpleUsecase::parse_time(record.end)?;
        let record = domain::time_record::TimeRecord::new_with_id(
            self.id_generator.generate(),
            start,
            end,
            record.memo,
        );
        self.repository.create(record)?;
        Ok(())
    }

    fn delete_time_record(&mut self, id: String) -> Result<()> {
        self.repository.delete(id)?;
        Ok(())
    }

    fn list_time_records(&self) -> Result<Vec<TimeRecordWithID>> {
        todo!()
    }
}


