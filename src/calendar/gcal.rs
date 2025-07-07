use anyhow::anyhow;
use icalendar::parser::{self, Calendar};

pub fn parse_gcal(contents: &str) -> anyhow::Result<Calendar> {
    match parser::read_calendar(contents) {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!(e)),
    }
}
