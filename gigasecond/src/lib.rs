extern crate chrono;

use chrono::*;

const SECONDS_ADD: i64 = 1000000000;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let new_date: DateTime<UTC> = date + Duration::seconds(SECONDS_ADD);
    new_date
}
