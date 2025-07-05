use std::io::BufRead;

use chrono::{DateTime, Utc};

pub struct CalendarEvent {
    pub uid: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub status: EventStatus,
    // pub recurrence_rule: Option<String>,
    // pub attendees: Vec<String>, todo
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventStatus {
    Confirmed,
    Cancelled,
    Tentative,
    Unknown,
}

pub struct Calendar {
    pub name: String,
    pub events: Vec<CalendarEvent>,
}

/// Trait for calendar data sources (Google Calendar, Apple Calendar, etc)
pub trait CalendarSource {
    /// Load calendar data from some input (file path, URL, etc)
    fn load(&self) -> anyhow::Result<Calendar>;
}

/// Trait for parsing iCalendar data (the .ics format)
pub trait IcalParser<B: BufRead> {
    /// Parse raw iCal data into Calendar struct
    fn parse(&self, r: B) -> anyhow::Result<Calendar>;
}

/// Trait for writing calendar data back to a format (e.g. .ics)
pub trait CalendarWriter {
    fn write(&self, calendar: &Calendar) -> anyhow::Result<String>;
}

/// A scheduler or query interface to find events by date/time
pub trait CalendarQuery {
    /// Return all events overlapping given datetime range
    fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<CalendarEvent>;

    /// Return next upcoming event after given time
    fn next_event_after(&self, after: DateTime<Utc>) -> Option<CalendarEvent>;
}
