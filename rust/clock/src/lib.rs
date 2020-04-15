use std::fmt;

#[derive(Debug)]
pub struct Clock {
    // Duration in minutes
    d: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.d >= 0 {
            let hours = self.d / 60 % 24;
            let minutes = self.d % 60;
            write!(f, "{:02}:{:02}", hours, minutes)
        } else {
            let minutes = (60 + (self.d % 60)) % 60;
            let base_hour = if minutes != 0 && minutes < 60 { 23 } else { 24 };
            let hours = (base_hour + (self.d / 60 % 24)) % 24;
            write!(f, "{:02}:{:02}", hours, minutes)
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            d: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
