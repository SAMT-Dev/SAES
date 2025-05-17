use chrono::{Datelike, Duration, Local, NaiveDateTime, NaiveTime};

use crate::BASE_HASHMAP;

pub struct Friday {
    pub last_friday: NaiveDateTime,
    pub before_last_friday: NaiveDateTime,
    pub next_friday: NaiveDateTime,
}

pub fn get_fridays() -> Friday {
    let input_date = Local::now().date_naive();
    let current_weekday = input_date.weekday().num_days_from_monday();
    let week_close = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
    // Calculate the number of days until the next Friday (5th day of the week)
    let days_until_friday = (4 + 7 - current_weekday) % 7;
    // If the input date is already Friday, we need to move to the next week's Friday
    let days_until_friday = if current_weekday == 4 {
        0
    } else {
        days_until_friday
    };

    // Get the next Friday by adding the days until Friday to the input date
    let next_friday = input_date + Duration::days(days_until_friday.into());
    let last_friday = next_friday + Duration::days(-7);
    let laster_friday = last_friday + Duration::days(-7);
    let next_friday_whole = NaiveDateTime::new(next_friday, week_close);
    let last_friday_whole = NaiveDateTime::new(last_friday, week_close);
    let laster_friday_whole = NaiveDateTime::new(laster_friday, week_close);
    Friday {
        last_friday: last_friday_whole,
        before_last_friday: laster_friday_whole,
        next_friday: next_friday_whole,
    }
}

#[derive(Debug, PartialEq)]
pub enum EnvModes {
    Production,
    Testing,
    Devel,
}

pub async fn get_env_mode() -> EnvModes {
    let hash = BASE_HASHMAP.read().await;
    let mode = hash.get("env_mode").unwrap();
    match mode.to_lowercase().as_str() {
        "production" => EnvModes::Production,
        "testing" => EnvModes::Testing,
        &_ => EnvModes::Devel,
    }
}
