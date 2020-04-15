use std::fmt;

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Clock {
            minutes: Self::to_clock_minutes((HOUR * hours) + minutes),
        }
    }
    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.minutes = Self::to_clock_minutes(&self.minutes + minutes);
        self
    }

    pub fn hours(&self) -> i32 {
        self.minutes / HOUR
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % HOUR
    }

    fn to_clock_minutes(minutes: i32) -> i32 {
        (DAY + (minutes % DAY)) % DAY
    }
}
