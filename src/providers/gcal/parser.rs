use chrono::{DateTime, TimeZone, Utc};
use ical::parser::ical;
use std::{io::BufRead, str::FromStr};

use anyhow::{Result, anyhow};

use crate::common::calendar::{Calendar, CalendarEvent, EventStatus, IcalParser};

pub(super) struct GcalParser {}

impl GcalParser {
    pub(super) fn new() -> GcalParser {
        Self {}
    }
}

impl<B: BufRead> IcalParser<B> for GcalParser {
    fn parse(&self, r: B) -> anyhow::Result<Calendar> {
        let parser = ical::IcalParser::new(r);
        let mut events = Vec::new();

        for calendar in parser {
            let calendar = calendar?;

            for component in calendar.events {
                let mut uid = None;
                let mut summary = None;
                let mut description = None;
                let mut location = None;
                let mut dtstart = None;
                let mut dtend = None;
                let mut status = None;

                for prop in component.properties {
                    match prop.name.as_str() {
                        "UID" => uid = prop.value,
                        "SUMMARY" => summary = prop.value,
                        "DESCRIPTION" => description = prop.value,
                        "LOCATION" => location = prop.value,
                        "DTSTART" => dtstart = prop.value,
                        "DTEND" => dtend = prop.value,
                        "STATUS" => status = prop.value,
                        _ => {}
                    }
                }

                let uid = uid.ok_or_else(|| anyhow!("Missing UID in event"))?;
                let start = dtstart
                    .as_deref()
                    .ok_or_else(|| anyhow!("Missing DTSTART in event {}", uid))
                    .and_then(parse_datetime)?;
                let end = dtend
                    .as_deref()
                    .ok_or_else(|| anyhow!("Missing DTEND in event {}", uid))
                    .and_then(parse_datetime)?;

                let event = CalendarEvent {
                    uid,
                    summary,
                    description,
                    location,
                    start,
                    end,
                    status: EventStatus::Unknown, // todo
                };

                events.push(event);
            }
        }

        Ok(Calendar {
            name: String::from_str("todo")?,
            events,
        })
    }
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>> {
    // Parse both
    // - UTC: 20250705T130000Z
    // - Local time: 20250705T150000
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = DateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ") {
        Ok(dt.with_timezone(&Utc))
    } else if let Ok(dt) = DateTime::parse_from_str(value, "%Y%m%dT%H%M%S") {
        // No TZ info - assume UTC or local depending on your app
        Ok(Utc.from_utc_datetime(&dt.naive_utc()))
    } else {
        Err(anyhow!("Failed to parse datetime: {}", value))
    }
}
