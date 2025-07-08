use anyhow::anyhow;
use icalendar::{
    Calendar,
    parser::{self},
};

pub fn parse_gcal(contents: &str) -> anyhow::Result<Calendar> {
    match parser::read_calendar(contents) {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(anyhow!(e)),
    }
}
