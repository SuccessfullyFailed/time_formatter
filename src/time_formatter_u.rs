#[cfg(test)]
mod tests {
	use std::time::SystemTime;
	use crate::TimeFormatter;



	fn test_date(year:u16, months:u8, days:u8, hours:u8, minutes:u8, seconds:u8) {
		let formated_time:TimeFormatter = TimeFormatter::new_date_raw(year, months, days, hours, minutes, seconds);
		assert_eq!(formated_time.year(), year);
		assert_eq!(formated_time.months(), months);
		assert_eq!(formated_time.days(), days);
		assert_eq!(formated_time.hours(), hours);
		assert_eq!(formated_time.minutes(), minutes);
		assert_eq!(formated_time.seconds(), seconds);
	}



	#[test]
	fn test_time() {
		for hours in 0..23 {
			for minutes in 0..60 {
				for seconds in 0..60 {
					test_date(0, 0, 0, hours, minutes, seconds);
				}
			}
		}
	}

	#[test]
	fn test_time_day() {
		for days in 0..31 {
			println!("{days}");
			test_date(0, 0, days, 8, 30, 15);
		}
	}

	#[test]
	fn test_time_day_month() {
		for months in 0..12 {
			println!("{months}");
			test_date(0, months, 4, 8, 30, 15);
		}
	}

	#[test]
	fn test_time_day_month_year() {
		for year in 0..2036 {
			println!("{year}");
			test_date(year, 8, 4, 8, 30, 15);
		}
	}

	#[test]
	fn test_daylight_savings() {
		for (month, date, hour, should_mod) in [(3, 2, 0, false), (3, 2, 1, true), (4, 5, 6, true), (5, 1, 0, true), (5, 6, 0, true), (5, 6, 6, true), (5, 6, 7, false), (8, 1, 0, false)] {
			let formatter:TimeFormatter = TimeFormatter::new_date(0, month, date, hour, 0, 0).with_daylight_saving_time(3, 2, 1, 5, 6, 7, 12);
			assert_eq!(formatter.hours(), hour + if should_mod { 12 } else { 0 });
		}
	}

	#[test]
	fn test_daylight_savings_flipped() {
		for (month, date, hour, should_mod) in [(3, 2, 0, false), (3, 2, 1, true), (4, 5, 6, true), (5, 1, 0, true), (5, 6, 0, true), (5, 6, 6, true), (5, 6, 7, false), (8, 1, 0, false)] {
			let formatter:TimeFormatter = TimeFormatter::new_date(0, month, date, hour, 0, 0).with_daylight_saving_time(5, 6, 7, 3, 2, 1, 12);
			assert_eq!(formatter.hours(), hour + if should_mod { 0 } else { 12 });
		}
	}
}