use crate::dates::{parse_hm, Date};
use crate::frontmatter::Session;
use std::collections::BTreeMap;

pub struct DlogEntry {
    pub date: Date,
    pub title: String,
    pub goal: String,
    pub summary: String,
    pub sessions: Vec<Session>,
    pub body_html: String,
    pub slug: String,
}

impl DlogEntry {
    pub fn display_title(&self) -> &str {
        if !self.title.is_empty() {
            &self.title
        } else {
            &self.goal
        }
    }

    pub fn tags(&self) -> Vec<String> {
        let mut seen = Vec::new();
        for s in &self.sessions {
            for t in &s.tags {
                if !seen.contains(t) {
                    seen.push(t.clone());
                }
            }
        }
        seen
    }
}

fn session_minutes(s: &Session) -> i64 {
    parse_hm(&s.time_out) - parse_hm(&s.time_in)
}

pub struct Stats {
    pub num_days: i64,
    pub num_documented_days: usize,
    pub percentage_documented: i64,
    pub total_hrs: i64,
    pub total_mins: i64,
    pub streak: i64,
    /// (tag, minutes, size_em, color) sorted by tag name.
    pub tags: Vec<(String, i64, f64, &'static str)>,
}

const PALETTE: [&str; 8] = [
    "#c0392b", "#2471a3", "#1e8449", "#af601a", "#7d3c98", "#117864", "#b03a5b", "#4a5a8a",
];

pub fn compute(entries: &[DlogEntry], start: Date, today: Date) -> Stats {
    let num_days = (today.days_since_epoch() - start.days_since_epoch()) + 1;
    let num_documented_days = entries.len();
    let percentage_documented = if num_days > 0 {
        (num_documented_days as i64 * 100) / num_days
    } else {
        0
    };

    let total_minutes: i64 = entries
        .iter()
        .flat_map(|e| e.sessions.iter())
        .map(session_minutes)
        .sum();
    let total_hrs = total_minutes / 60;
    let total_mins = total_minutes % 60;

    let mut minutes_by_tag: BTreeMap<String, i64> = BTreeMap::new();
    for e in entries {
        for s in &e.sessions {
            for t in &s.tags {
                *minutes_by_tag.entry(t.clone()).or_insert(0) += session_minutes(s);
            }
        }
    }
    let max_minutes = minutes_by_tag.values().copied().max().unwrap_or(0);
    let tags: Vec<(String, i64, f64, &'static str)> = minutes_by_tag
        .into_iter()
        .enumerate()
        .map(|(idx, (tag, mins))| {
            let ratio = if max_minutes > 0 {
                (mins * 100) / max_minutes
            } else {
                0
            };
            let size = ((ratio as f64 / 100.0 * 1.6 + 0.85) * 100.0).round() / 100.0;
            (tag, mins, size, PALETTE[idx % PALETTE.len()])
        })
        .collect();

    // Streak: consecutive days ending at the most recent entry.
    let mut sorted: Vec<&DlogEntry> = entries.iter().collect();
    sorted.sort_by_key(|e| std::cmp::Reverse(e.date));
    let mut streak = 0i64;
    if let Some(first) = sorted.first() {
        streak = 1;
        let mut prev = first.date;
        for e in sorted.iter().skip(1) {
            if prev.days_since_epoch() - e.date.days_since_epoch() == 1 {
                streak += 1;
                prev = e.date;
            } else {
                break;
            }
        }
    }

    Stats {
        num_days,
        num_documented_days,
        percentage_documented,
        total_hrs,
        total_mins,
        streak,
        tags,
    }
}
