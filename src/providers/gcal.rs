use anyhow::Result;
use std::path::PathBuf;

use crate::{
    common::calendar::{Calendar, CalendarSource},
    providers::gcal::src::GcalSrc,
};

mod parser;
mod src;

pub fn import_gcal(path: PathBuf) -> Result<Calendar> {
    let src = GcalSrc::new(path);
    src.load()
}
