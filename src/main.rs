use std::{ops::Add, time::Duration};

use chrono::prelude::*;
use nameof::name_of;
fn main() {
    let utc_now = Utc::now();

    let local_now = create_local_from_utc(&utc_now);
    println!("{}: {}", name_of!(utc_now), &utc_now);
    println!("{}: {}", name_of!(local_now), &local_now);
}

fn create_local_from_utc(utc: &DateTime<Utc>) -> DateTime<Local> {
    utc.with_timezone(&Local)
}
