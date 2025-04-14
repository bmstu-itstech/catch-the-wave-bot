use chrono::{Datelike, Local};

use crate::domain::interfaces::WeekService;
use crate::domain::models::WeekId;


#[derive(Default)]
pub struct ChronoWeekService;

impl WeekService for ChronoWeekService {
    fn current(&self) -> WeekId {
        let today = Local::now();
        let iso_week = today.iso_week();
        WeekId {
            year: iso_week.year(),
            week: iso_week.week(),
        }
    }

    fn next(&self, week_id: WeekId) -> WeekId {
        let date = chrono::NaiveDate::from_isoywd_opt(week_id.year, week_id.week, chrono::Weekday::Mon)
            .unwrap_or_else(|| {
                chrono::NaiveDate::from_isoywd_opt(week_id.year + 1, 1, chrono::Weekday::Mon).unwrap()
            });

        let next_date = date + chrono::Duration::weeks(1);
        let iso_week = next_date.iso_week();

        WeekId {
            year: iso_week.year(),
            week: iso_week.week(),
        }
    }
}
