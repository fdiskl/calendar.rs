use crate::{
    common::calendar::{Calendar, CalendarSource, IcalParser},
    providers::gcal::parser::GcalParser,
};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

pub(super) struct GcalSrc {
    ics_path: PathBuf,
}

impl GcalSrc {
    pub(super) fn new(ics_path: PathBuf) -> Self {
        Self { ics_path }
    }
}

impl CalendarSource for GcalSrc {
    fn load(&self) -> anyhow::Result<Calendar> {
        let r = BufReader::new(File::open(&self.ics_path)?);
        let parser = GcalParser::new();

        parser.parse(r)
    }
}
