//! Deserialize a `std::time::Duration` from a string like "1ms", "1s", "1us", "1ns"

use std::time::Duration;

use serde::{de, Deserialize, Deserializer};

fn parse_failed(s: &str) -> String {
    format!("failed to parse as Duration: {s}")
}

fn duration_from_str(s: &str) -> Option<Duration> {
    let t = s.trim();

    if t.ends_with("ns") {
        t[0..t.len() - 2]
            .trim()
            .parse::<u64>()
            .map(Duration::from_nanos)
            .ok()
    } else if t.ends_with("us") {
        t[0..t.len() - 2]
            .trim()
            .parse::<u64>()
            .map(Duration::from_micros)
            .ok()
    } else if t.ends_with("ms") {
        t[0..t.len() - 2]
            .trim()
            .parse::<u64>()
            .map(Duration::from_millis)
            .ok()
    } else if t.ends_with('s') {
        t[0..t.len() - 1]
            .trim()
            .parse::<u64>()
            .map(Duration::from_secs)
            .ok()
    } else {
        None
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    duration_from_str(&s).ok_or_else(|| de::Error::custom(parse_failed(&s)))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::serde_duration::duration_from_str;

    #[test]
    fn nanos() {
        assert_eq!(Some(Duration::from_nanos(10)), duration_from_str("10ns"));
        assert_eq!(Some(Duration::from_nanos(10)), duration_from_str("10 ns"));
        assert_eq!(Some(Duration::from_nanos(10)), duration_from_str(" 10 ns "));
    }

    #[test]
    fn micros() {
        assert_eq!(Some(Duration::from_micros(10)), duration_from_str("10us"));
        assert_eq!(Some(Duration::from_micros(10)), duration_from_str("10 us"));
        assert_eq!(
            Some(Duration::from_micros(10)),
            duration_from_str(" 10 us ")
        );
    }

    #[test]
    fn millis() {
        assert_eq!(Some(Duration::from_millis(10)), duration_from_str("10ms"));
        assert_eq!(Some(Duration::from_millis(10)), duration_from_str("10 ms"));
        assert_eq!(
            Some(Duration::from_millis(10)),
            duration_from_str(" 10 ms ")
        );
    }

    #[test]
    fn secs() {
        assert_eq!(Some(Duration::from_secs(10)), duration_from_str("10s"));
        assert_eq!(Some(Duration::from_secs(10)), duration_from_str("10 s"));
        assert_eq!(Some(Duration::from_secs(10)), duration_from_str(" 10 s "));
    }

    #[test]
    fn none() {
        assert_eq!(None, duration_from_str("10"));
        assert_eq!(None, duration_from_str("10 m s"));
        assert_eq!(None, duration_from_str(" abc"));
    }
}
