use std::{ fmt::{Debug, Display}, sync::OnceLock, time::{ Duration, SystemTime, UNIX_EPOCH } };



pub(crate) const SECONDS_PER_MINUTE:u64 = 60;
pub(crate) const SECONDS_PER_HOUR:u64 = 60 * SECONDS_PER_MINUTE;
pub(crate) const SECONDS_PER_DAY:u64 = 24 * SECONDS_PER_HOUR;
pub(crate) const SECONDS_PER_MONTH:fn(month_index:u64, is_leap_year:bool) -> u64 = |month_index, is_leap_year| if month_index == 1 { if is_leap_year { 29 } else { 28 } } else { 30 + ((month_index + (if month_index > 6 { 0 } else { 1 })) % 2) } * SECONDS_PER_DAY;
pub(crate) const SECONDS_PER_YEAR:fn(is_leap_year:bool) -> u64 = |is_leap_year| if is_leap_year { 366 } else { 365 } * SECONDS_PER_DAY;



pub struct TimeFormatter {
	seconds_since_y0:u64,
	year:u16,
	months:u8,
	days:u8,
	hours:u8,
	minutes:u8,
	seconds:u8
}
impl TimeFormatter {

	/* CONSTRUCTOR METHODS */

	/// Create a new formatter from a supported time-format.
	pub fn new<T:TimeFormattable>(time:T) -> TimeFormatter {
		let seconds_since_y0:u64 = time.seconds_since_y0();
		let mut time:TimeFormatter = TimeFormatter { seconds_since_y0, year: 0, months: 0, days: 0, hours: 0, minutes: 0, seconds: 0 };
		time.parse_formatted();
		time
	}

	/// Create a new formatter from separate date parts. Month and date should start at 1.
	pub fn new_date(year:u16, month:u8, date:u8, hours:u8, minutes:u8, seconds:u8) -> TimeFormatter {
		TimeFormatter::new_date_raw(year, month - 1, date - 1, hours, minutes, seconds)
	}

	/// Return self with daylight saving time enabled.
	pub fn with_daylight_saving_time(mut self, start_month:u8, start_date:u8, start_hour:u8, end_month:u8, end_date:u8, end_hour:u8, hour_modification:i8) -> Self {
		let savings:DaylightSavings = DaylightSavings::new(start_month, start_date, start_hour, end_month, end_date, end_hour, hour_modification);
		let modification:i8 = savings.get_modification(self.months, self.days, self.hours);
		if modification != 0 {
			self.seconds_since_y0 = ((self.seconds_since_y0 as i64) + (modification as i64 * SECONDS_PER_HOUR as i64)) as u64;
		}
		self.parse_formatted();
		self
	}

	/// Create a new formatter from raw date parts. Month and date should start at 0.
	pub fn new_date_raw(year:u16, months:u8, days:u8, hours:u8, minutes:u8, seconds:u8) -> TimeFormatter {
		let (year_u64, months_u64, days_u64, hours_u64, minutes_u64, seconds_u64) = (year as u64, months as u64, days as u64, hours as u64, minutes as u64, seconds as u64);
		let leap_days:u64 = if year_u64 == 0 { 0 } else { (year_u64 - 1) / 4 + 1 }; // Leap years should be added once the year is over. 1 year passed means 1 leap year.

		let days_in_years:u64 = year_u64 * 365 + leap_days;
		let days_in_month:u64 = (0..months_u64).map(|month_index| if month_index == 1 { if year_u64 % 4 == 0 { 29 } else { 28 } } else { 30 + ((month_index + (if month_index > 6 { 0 } else { 1 })) % 2) }).sum::<u64>();
		let seconds_since_y0:u64 = (((days_in_years + days_in_month + days_u64) * 24 + hours_u64) * 60 + minutes_u64) * 60 + seconds_u64;

		TimeFormatter::new(seconds_since_y0)
	}



	/* PARSING METHODS */

	/// Parse the formatted properties from the seconds since year 0.
	fn parse_formatted(&mut self) {
		let mut remaining_seconds:u64 = self.seconds_since_y0;

		self.year = 0;
		while Self::take_formatted_time(&mut remaining_seconds, SECONDS_PER_YEAR(self.is_leap_year())) {
			self.year += 1;
		}
		let is_leap_year:bool = self.is_leap_year();
		
		self.months = 0;
		while Self::take_formatted_time(&mut remaining_seconds, SECONDS_PER_MONTH(self.months as u64, is_leap_year)) {
			self.months += 1;
		}

		self.days = (remaining_seconds / SECONDS_PER_DAY) as u8;
		remaining_seconds %= SECONDS_PER_DAY;

		self.hours = (remaining_seconds / SECONDS_PER_HOUR) as u8;
		remaining_seconds %= SECONDS_PER_HOUR;

		self.minutes = (remaining_seconds / SECONDS_PER_MINUTE) as u8;
		remaining_seconds %= SECONDS_PER_MINUTE;

		self.seconds = remaining_seconds as u8;
	}

	/// Remove an amount from time from the remaining seconds. Returns false if there was not enough.
	fn take_formatted_time(remaining_seconds_ref:&mut u64, seconds_to_take:u64) -> bool {
		if *remaining_seconds_ref > seconds_to_take {
			*remaining_seconds_ref -= seconds_to_take;
			true
		} else {
			false
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the year.
	pub fn year(&self) -> u16 {
		self.year
	}

	/// Whether or not this is a leap year.
	pub fn is_leap_year(&self) -> bool {
		self.year % 4 == 0
	}

	/// Get the amount of months. Will return a value between 0 and 11 where 0 is the first month.
	pub fn months(&self) -> u8 {
		self.months
	}

	/// Get the month. Will return a value between 1 and 12 where 1 is the first month.
	pub fn month(&self) -> u8 {
		self.months + 1
	}
	
	/// Get the amount of days. Will return a value between 0 and 30 where 0 is the first day.
	pub fn days(&self) -> u8 {
		self.days
	}
	
	/// Get the day. Will return a value between 1 and 31 where 1 is the first day.
	pub fn date(&self) -> u8 {
		self.days + 1
	}

	/// Get the hours.
	pub fn hours(&self) -> u8 {
		self.hours
	}

	/// Get the minutes.
	pub fn minutes(&self) -> u8 {
		self.minutes
	}

	/// Get the seconds.
	pub fn seconds(&self) -> u8 {
		self.seconds
	}
}
impl PartialEq for TimeFormatter {
	fn eq(&self, other:&Self) -> bool {
		self.seconds_since_y0 == other.seconds_since_y0
	}
}
impl PartialOrd for TimeFormatter {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.seconds_since_y0.partial_cmp(&other.seconds_since_y0)
	}
}
impl Display for TimeFormatter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year(), self.month(), self.date(), self.hours(), self.minutes(), self.seconds())
	}
}
impl Debug for TimeFormatter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.year(), self.month(), self.date(), self.hours(), self.minutes(), self.seconds())
	}
}



pub trait TimeFormattable {
	fn seconds_since_y0(&self) -> u64;
}
impl TimeFormattable for TimeFormatter {
	fn seconds_since_y0(&self) -> u64 {
		self.seconds_since_y0
	}
}
impl TimeFormattable for SystemTime {
	fn seconds_since_y0(&self) -> u64 {
		static VALUE_CACHE:OnceLock<u64> = OnceLock::new();
		let seconds_until_1970:&u64 = VALUE_CACHE.get_or_init(|| (0..1970).map(|year| SECONDS_PER_YEAR(year % 4 == 0)).sum());
		seconds_until_1970 + self.duration_since(UNIX_EPOCH).unwrap().as_secs()
	}
}
impl TimeFormattable for Duration {
	fn seconds_since_y0(&self) -> u64 {
		self.as_secs()
	}
}
impl TimeFormattable for u64 {
	fn seconds_since_y0(&self) -> u64 {
		*self
	}
}



struct DaylightSavings {
	start_month_index:u8,
	start_day_index:u8,
	start_hour:u8,

	end_month_index:u8,
	end_day_index:u8,
	end_hour:u8,

	hour_modification:i8,
	flipped:bool
}
impl DaylightSavings {

	/// Create a new daylight savings set.
	fn new(start_month:u8, start_date:u8, start_hour:u8, end_month:u8, end_date:u8, end_hour:u8, hour_modification:i8) -> DaylightSavings {
		let flipped:bool = (end_month < start_month) || (end_month == start_month && end_date < start_date) || (end_month == start_month && end_date == start_date && end_hour < start_hour);
		if !flipped {
			DaylightSavings {
				start_month_index: start_month - 1,
				start_day_index: start_date - 1,
				start_hour: start_hour,

				end_month_index: end_month - 1,
				end_day_index: end_date - 1,
				end_hour: end_hour,

				hour_modification,
				flipped
			}
		} else {
			DaylightSavings {
				end_month_index: start_month - 1,
				end_day_index: start_date - 1,
				end_hour: start_hour,

				start_month_index: end_month - 1,
				start_day_index: end_date - 1,
				start_hour: end_hour,

				hour_modification,
				flipped
			}
		}
	}

	/// Whether or not the modification should be applied.
	fn should_apply_modification(&self, month_index:u8, day_index:u8, hour:u8) -> bool {

		// Return false if it's too early.
		if month_index < self.start_month_index {
			return false;
		} else if month_index == self.start_month_index {
			if day_index < self.start_day_index {
				return false;
			} else if day_index == self.start_day_index && hour < self.start_hour {
				return false;
			}
		}
		
		// Return false if it's too late.
		if month_index > self.end_month_index {
			return false;
		} else if month_index == self.end_month_index {
			if day_index > self.end_day_index {
				return false;
			} else if day_index == self.end_day_index && hour >= self.end_hour {
				return false;
			}
		}

		// No problems found, return true.
		true
	}

	/// Get the modification for a specific date.
	fn get_modification(&self, month_index:u8, day_index:u8, hour:u8) -> i8 {
		let should_apply:bool = self.should_apply_modification(month_index, day_index, hour);
		if should_apply != self.flipped {
			self.hour_modification
		} else {
			0
		}
	}
}