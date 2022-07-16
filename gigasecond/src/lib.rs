use std::ops::Add;
use std::time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start.add(Duration::from_secs(1_000_000_000));
}
