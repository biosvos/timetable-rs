use std::{error, fmt};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "just error")
    }
}

impl Error for MyError {}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Usecase {
    fn create_time_record(&mut self, record: TimeRecord) -> Result<()>;
    fn delete_time_record(&mut self, id: String) -> Result<()>;
    fn list_time_records(&self) -> Result<Vec<TimeRecordWithID>>;
}

#[derive(Clone)]
pub struct TimeRecordWithID {
    pub id: String,
    pub start: String,
    pub end: String,
    pub memo: String,
}

pub struct TimeRecord {
    pub start: String,
    pub end: String,
    pub memo: String,
}