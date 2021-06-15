use std::fmt;

#[derive(Debug , PartialEq, Eq)]
pub struct Clock {
    minutes: i64,
}

// fn format_hours(integer: i32) -> String {
//     let mut hours = integer;

//     if hours < 0 {
//         let diff = (24 + hours) % 24;
//         hours = 24 + diff;
//     }

//     if hours >= 24 {
//         hours %= 24;
//     } 
    
//     if hours < 10 {
//         format!("0{}", hours )
//     } else {
//         hours.to_string()
//     }
// }

// fn format_minutes(integer: i32) -> String {
//     if integer < 10 {
//         format!("0{}", integer)
//     } else {
//         integer.to_string()
//     }
// }

// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         let mut hours = self.0;
//         let mut minutes = self.1;

//         // handle minutes roll over
//         if minutes >= 60 {
//             let hours_from_minutes = minutes / 60;
//             let leftover_minutes = minutes % 60;

//             hours += hours_from_minutes;
//             minutes = leftover_minutes;
//         }

//         // handle negative minutes
//         if minutes < 0 {
//             if minutes < -60 {
//                 let hours_from_minutes = minutes / 60;
//                 let leftover_minutes = minutes % 60;
    
//                 if leftover_minutes < 0 {
//                     hours -= 1;    
//                 }
    
//                 hours += hours_from_minutes;
//                 minutes = (60 + leftover_minutes) % 60;
//             } else {
//                 hours -= 1;
//                 minutes += 60;
//             }
//         }

//         format_hours(hours) + ":" + &format_minutes(minutes)
//     }
// }

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.to_string() == other.to_string()
//     }
// }

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Self {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
