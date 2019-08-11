use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let timestamp = start.timestamp();
    Utc.timestamp(timestamp + (10 as i64).pow(9), 0)
}
