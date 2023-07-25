#![allow(dead_code)]
use chrono::{NaiveDate, Utc};

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

fn now_date() -> NaiveDate {
    NaiveDate::parse_from_str(NOW, "%Y-%m-%d").unwrap_or(Utc::now().date_naive())
}

struct User {
    birthdate: NaiveDate,
}

impl User {
    fn with_birthdate(birthdate: NaiveDate) -> Self {
        User { birthdate }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let years = match now_date().years_since(self.birthdate) {
            Some(y) => y,
            None => return 0,
        };
        // Either we get a valid amount of years, or the value didn't fit in and
        // the user probably comes from some ancient, mythological civilization.
        // At any rate, if we only have u16 to represent years, we're gonna use
        // it to its fullest.
        years.try_into().unwrap_or(u16::MAX)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    struct DateAndAge {
        date: (i32, u32, u32),
        age: u32,
    }

    #[test]
    fn counts_age() {
        let test_values = vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ];
        for ((y, m, d), expected) in test_values {
            let date = NaiveDate::from_ymd_opt(y, m, d).expect("Invalid date");
            let user = User::with_birthdate(date);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            // How does this make sense?
            // ((2016, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let date = NaiveDate::from_ymd_opt(y, m, d).expect("Invalid date");
            let user = User::with_birthdate(date);
            assert_eq!(user.age(), expected);
        }
    }
}
