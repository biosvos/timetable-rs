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
}

const DEFAULT_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

impl SimpleUsecase {
    pub fn new(repository: Box<dyn Repository>) -> SimpleUsecase {
        SimpleUsecase {
            repository,
        }
    }

    fn parse_time(time: String) -> Result<DateTime<Local>> {
        let datetime = Local.datetime_from_str(&time, DEFAULT_TIME_FORMAT)?;
        Ok(DateTime::from(datetime))
    }
}

impl Usecase for SimpleUsecase {
    fn create_time_record(&mut self, record: TimeRecordWithID) -> Result<()> {
        let start = SimpleUsecase::parse_time(record.start)?;
        let end = SimpleUsecase::parse_time(record.end)?;
        let record = domain::time_record::TimeRecord::new_with_id(
            record.id,
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

    fn list_time_records(&mut self) -> Result<Vec<TimeRecordWithID>> {
        let ret = self.repository.list()?;
        Ok(ret.iter().map(|x| TimeRecordWithID {
            id: x.id().to_string(),
            start: x.start().format(DEFAULT_TIME_FORMAT).to_string(),
            end: x.end().format(DEFAULT_TIME_FORMAT).to_string(),
            memo: x.memo().to_string(),
        }).collect())
    }
}