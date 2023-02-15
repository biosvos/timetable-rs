use std::fmt;
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
    fn create_time_record(&self, record: TimeRecordWithID) -> Result<()>;
    fn delete_time_record(&self, id: String) -> Result<()>;
    fn list_time_records(&self) -> Result<Vec<TimeRecordWithID>>;
}

#[derive(Clone)]
pub struct TimeRecordWithID {
    pub id: String,
    pub start: String,
    pub end: String,
    pub memo: String,
}