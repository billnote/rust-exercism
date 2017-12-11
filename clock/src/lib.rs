use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    hour: u8,
    minute: u8,
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        let mut total_minute = (hour * 60 + minute) % 1440;
        if total_minute < 0 {
            total_minute += 1440;
        }

        Clock {
            hour: (total_minute / 60) as u8,
            minute: (total_minute % 60) as u8,
        }
    }

    pub fn add_minutes(&self, minute: i32) -> Clock {
        Clock::new(self.hour as i32, (self.minute as i32) + minute)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
