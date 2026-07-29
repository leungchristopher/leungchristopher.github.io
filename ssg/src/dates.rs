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

    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}T00:00:00+00:00", self.y, self.m, self.d)
    }

    // Howard Hinnant's days-from-civil, days since 1970-01-01.
    pub fn days_since_epoch(&self) -> i64 {
        let y = if self.m <= 2 { self.y as i64 - 1 } else { self.y as i64 };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = (y - era * 400) as i64;
        let mp = (self.m as i64 + 9) % 12;
        let doy = (153 * mp + 2) / 5 + self.d as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146097 + doe - 719468
    }

    // Howard Hinnant's civil-from-days, inverse of the above.
    pub fn from_days_since_epoch(z: i64) -> Date {
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
        Date::from_days_since_epoch((secs / 86400) as i64)
    }
}

/// Parses "HH:MM" into minutes since midnight.
pub fn parse_hm(s: &str) -> i64 {
    let (h, m) = s.split_once(':').unwrap_or(("0", "0"));
    h.parse::<i64>().unwrap_or(0) * 60 + m.parse::<i64>().unwrap_or(0)
}
