use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

fn normalize(mut minutes: i32) -> i32 {
    while minutes < 0 { minutes += 1440; }
    minutes % 1440
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes:  normalize(minutes + hours * 60) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { minutes: normalize(self.minutes + minutes)}
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes %60)
    }
}