use std::fmt;

#[derive(Debug)]
pub struct Clock {
    // Duration in minutes
    d: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{}", self.d);
        let hours = self.d / 60 % 24;
        let minutes = self.d % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
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
