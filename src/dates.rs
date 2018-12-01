use chrono::prelude::*;

pub struct Dates {
	pub dates_next_month: Vec<u32>,
	pub dates_current_month: Vec<u32>,
	pub dates_prv_month: Vec<u32>,
	pub weekday: u32,
	pub weekday_name: String,
	pub month: u32,
	pub month_str: String,
	pub year: i32,
}

impl Dates {
	pub fn new() -> Dates {
		let dt = Local::now();

		Dates {
			dates_next_month: Dates::dates_month(dt.year(), dt.month() - 1),
			dates_current_month: Dates::dates_month(dt.year(), dt.month()),
			dates_prv_month: Dates::dates_month(dt.year(), dt.month() + 1),
			weekday: dt.weekday().number_from_monday(),
			weekday_name: dt.format("%a").to_string(),
			month: dt.month(),
			month_str: dt.format("%B").to_string(),
			year: dt.year(),
		}
	}

	fn dates_month(year: i32, month: u32) -> Vec<u32> {
		let mut dates_of_month = Vec::new();

		for date in 1..Dates::last_day_of_month(year, month) {
			dates_of_month.push(date);
		}

		dates_of_month
	}

	fn is_leap_year(year: i32) -> bool {
		NaiveDate::from_ymd_opt(year, 2, 29).is_some()
	}

	fn last_day_of_month(year: i32, month: u32) -> u32 {
		NaiveDate::from_ymd_opt(year, month + 1, 1)
			.unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
			.pred()
			.day()
	}
}
