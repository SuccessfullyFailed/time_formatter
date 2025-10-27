use std::{ sync::{ OnceLock }, time::{ Duration, SystemTime, UNIX_EPOCH } };



pub(crate) const SECONDS_PER_MINUTE:u64 = 60;
pub(crate) const SECONDS_PER_HOUR:u64 = 60 * SECONDS_PER_MINUTE;
pub(crate) const SECONDS_PER_DAY:u64 = 24 * SECONDS_PER_HOUR;
pub(crate) const SECONDS_PER_MONTH:fn(month_index:u64, is_leap_year:bool) -> u64 = |month_index, is_leap_year| if month_index == 1 { if is_leap_year { 28 } else { 29 } } else { 30 + ((month_index + (if month_index > 6 { 0 } else { 1 })) % 2) } * SECONDS_PER_DAY;
pub(crate) const SECONDS_PER_YEAR:fn(is_leap_year:bool) -> u64 = |is_leap_year| if is_leap_year { 366 } else { 365 } * SECONDS_PER_DAY;



pub struct TimeFormatter {
	_seconds_since_y0:u64,
	year:u16,
	months:u8,
	days:u8,
	hours:u8,
	minutes:u8,
	seconds:u8
}
impl TimeFormatter {

	/* CONSTRUCTOR METHODS */

	/// Create a new formatter from a SystemTime.
	pub fn new<T:TimeFormattable>(time:T) -> TimeFormatter {
		let seconds_since_y0:u64 = time.seconds_since_y0();
		let mut remaining_seconds:u64 = seconds_since_y0;

		let mut year:u64 = 0;
		while Self::take_formatted_time(&mut remaining_seconds, SECONDS_PER_YEAR(year % 4 == 0)) {
			year += 1;
		}
		let is_leap_year:bool = year % 4 == 0;
		
		let mut month_index:u64 = 0;
		while Self::take_formatted_time(&mut remaining_seconds, SECONDS_PER_MONTH(month_index, is_leap_year)) {
			month_index += 1;
		}

		let day_index:u64 = remaining_seconds / SECONDS_PER_DAY;
		remaining_seconds %= SECONDS_PER_DAY;

		let hours:u64 = remaining_seconds / SECONDS_PER_HOUR;
		remaining_seconds %= SECONDS_PER_HOUR;

		let minutes:u64 = remaining_seconds / SECONDS_PER_MINUTE;
		remaining_seconds %= SECONDS_PER_MINUTE;

		let seconds:u64 = remaining_seconds;

		TimeFormatter {
			_seconds_since_y0: seconds_since_y0,
			year: year as u16,
			months: month_index as u8,
			days: day_index as u8,
			hours: hours as u8,
			minutes: minutes as u8,
			seconds: seconds as u8
		}
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



pub trait TimeFormattable {
	fn seconds_since_y0(&self) -> u64;
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