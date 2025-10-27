#[cfg(test)]
mod tests {
	use crate::TimeFormatter;
	use std::time::Duration;



	fn test_date(year:u64, months:u64, days:u64, hours:u64, minutes:u64, seconds:u64) {
		let leap_years:u64 = if year == 0 { 0 } else { (year - 1) / 4 + 1 }; // Leap years should be added once the year is over. 1 year passed means 1 leap year.

		// Convert date to seconds past y0.
		let days_in_years:u64 = year * 365 + leap_years;
		let days_in_months:u64 = (0..months).map(|month_index| if month_index == 1 { if year % 4 == 0 { 28 } else { 29 } } else { 30 + ((month_index + (if month_index > 6 { 0 } else { 1 })) % 2) }).sum();
		let seconds_in_date:u64 = (((days_in_years + days_in_months + days) * 24 + hours) * 60 + minutes) * 60 + seconds;

		// Validate time formatter concludes original date.
		let formated_time:TimeFormatter = TimeFormatter::new(Duration::from_secs(seconds_in_date));
		assert_eq!(formated_time.year() as u64, year);
		assert_eq!(formated_time.months() as u64, months);
		assert_eq!(formated_time.days() as u64, days);
		assert_eq!(formated_time.hours() as u64, hours);
		assert_eq!(formated_time.minutes() as u64, minutes);
		assert_eq!(formated_time.seconds() as u64, seconds);
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
			test_date(0, 0, days, 8, 30, 15);
		}
	}

	#[test]
	fn test_time_day_month() {
		for months in 0..12 {
			test_date(0, months, 4, 8, 30, 15);
		}
	}

	#[test]
	fn test_time_day_month_year() {
		for year in 0..2036 {
			test_date(year, 8, 4, 8, 30, 15);
		}
	}
}