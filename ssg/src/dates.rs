// Minimal date handling: just enough to sort, format, and diff
// YYYY-MM-DD dates. No timezone/calendar library needed for that.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub y: i32,
    pub m: u32,
    pub d: u32,
}

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December",
];

impl Date {
    pub fn parse(s: &str) -> Option<Date> {
        let s = s.trim();
        let mut parts = s.split('-');
        let y = parts.next()?.parse().ok()?;
        let m = parts.next()?.parse().ok()?;
        let d = parts.next()?.parse().ok()?;
        Some(Date { y, m, d })
    }

    pub fn long(&self) -> String {
        format!("{} {} {}", self.d, MONTHS[(self.m - 1) as usize], self.y)
    }

    pub fn dotted(&self) -> String {
        format!("{:02}.{:02}.{:04}", self.d, self.m, self.y)
    }

    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}T00:00:00+00:00", self.y, self.m, self.d)
    }

    // Howard Hinnant's civil-from-days, inverse of the above.
    fn from_days(z: i64) -> Date {
        let z = z + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = if m <= 2 { y + 1 } else { y };
        Date { y: y as i32, m, d }
    }

    pub fn today() -> Date {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Date::from_days((secs / 86400) as i64)
    }
}
