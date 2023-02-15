use chrono::{DateTime, Local, TimeZone};
use crate::domain;
use crate::usecase::usecase::{TimeRecordWithID, Usecase, Result};

pub trait Repository {
    fn create(&self, record: domain::time_record::TimeRecord) -> Result<()>;
    fn delete(&self, id: String) -> Result<()>;
    fn list(&self) -> Result<Vec<domain::time_record::TimeRecord>>;
}

#[derive(Clone)]
pub struct SimpleUsecase<R: Repository> {
    repository: R,
}

const DEFAULT_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

impl<R> SimpleUsecase<R>
    where
        R: Repository
{
    pub fn new(repository: R) -> SimpleUsecase<R> {
        SimpleUsecase {
            repository,
        }
    }

    fn parse_time(time: String) -> Result<DateTime<Local>> {
        let datetime = Local.datetime_from_str(&time, DEFAULT_TIME_FORMAT)?;
        Ok(DateTime::from(datetime))
    }
}

impl<R> Usecase for SimpleUsecase<R>
    where
        R: Repository
{
    fn create_time_record(&self, record: TimeRecordWithID) -> Result<()> {
        let start = SimpleUsecase::<R>::parse_time(record.start)?;
        let end = SimpleUsecase::<R>::parse_time(record.end)?;
        let record = domain::time_record::TimeRecord::new_with_id(
            record.id,
            start,
            end,
            record.memo,
        );
        self.repository.create(record)?;
        Ok(())
    }

    fn delete_time_record(&self, id: String) -> Result<()> {
        self.repository.delete(id)?;
        Ok(())
    }

    fn list_time_records(&self) -> Result<Vec<TimeRecordWithID>> {
        let ret = self.repository.list()?;
        Ok(ret.iter().map(|x| TimeRecordWithID {
            id: x.id().to_string(),
            start: x.start().format(DEFAULT_TIME_FORMAT).to_string(),
            end: x.end().format(DEFAULT_TIME_FORMAT).to_string(),
            memo: x.memo().to_string(),
        }).collect())
    }
}