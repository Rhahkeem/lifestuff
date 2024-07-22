use super::DateTimeKeeper;
#[cfg(test)]

fn get_31_jan_2023() -> DateTimeKeeper {
    DateTimeKeeper::new_from_dmy_str("31 / 1/ 2023", false).unwrap()
}

mod tests {
    use crate::dateinfo::{datetimekeeper::tests::get_31_jan_2023, DateTimeKeeper};
    use time::{macros::date, Date};

    #[test]
    fn test_new_from_dmy() {
        let tester = DateTimeKeeper::new_from_dmy(31, 1, 2023);

        assert!(tester.is_ok());
        assert_eq!(tester.unwrap().date(), date!(2023 - 1 - 31));
    }

    #[test]
    fn test_new_from_dmy_str_passes() {
        let tester = DateTimeKeeper::new_from_dmy_str("31 / 1/ 2023", false);

        assert!(
            tester.is_ok(),
            "Assertion failed with Err:{:?}",
            tester.err()
        );

        let result = tester.unwrap();
        assert_eq!(result.date(), date!(2023 - 01 - 31))
    }

    #[test]
    fn test_new_from_dmy_str_fails() {
        let tester = DateTimeKeeper::new_from_dmy_str("32 / 1/ 2023", false);

        assert!(tester.is_err());
    }

    #[test]
    fn test_new_from_yyyymmdd_str_passes() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20230301", false);

        assert!(
            tester.is_ok(),
            "Assertion failed with Err:{:?}",
            tester.err()
        );

        let result = tester.unwrap();
        assert_eq!(result.date(), date!(2023 - 03 - 01))
    }

    #[test]
    fn test_new_from_yyyymmdd_str_fails() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("202303001", false);

        assert!(tester.is_err());
    }

    #[test]
    fn test_new_from_yyyymmdd_str_fails_bad_date() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20231301", false);

        assert!(tester.is_err());
    }

    #[test]
    fn test_set_date_ymd() {
        let mut tester = DateTimeKeeper::now();
        let test_date = tester.set_date_ymd(2023, &time::Month::March, 30);
        assert!(test_date.is_ok());
        assert_eq!(tester.date(), date!(2023 - 3 - 30));
    }

    #[test]
    fn test_set_date_ymd_fails_bad_date() {
        let mut tester = DateTimeKeeper::now();
        let test_date = tester.set_date_ymd(2023, &time::Month::February, 30);
        assert!(test_date.is_err());
    }

    #[test]
    fn test_negative_set_year_fails() {
        let mut tester = DateTimeKeeper::now();
        let test_result = tester.set_year(-2020);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_positive_set_year_passes() {
        let mut tester = DateTimeKeeper::now();
        let test_result = tester.set_year(2020);
        assert!(test_result.is_ok());
        assert_eq!(tester.date().year(), 2020);
    }

    #[test]
    fn test_set_month_fails_invalid_date() {
        let mut tester = get_31_jan_2023();
        let test_result = tester.set_month(time::Month::February);
        assert!(test_result.is_err());
    }
    #[test]
    fn test_set_month_as_num_fails_invalid_date() {
        let mut tester = get_31_jan_2023();
        let test_result = tester.set_month_num(4 /*April*/);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_month_passes() {
        let mut tester = get_31_jan_2023();
        let test_result = tester.set_month(time::Month::July);
        assert!(test_result.is_ok());
        assert_eq!(tester.date(), date!(2023 - 7 - 31));
    }

    #[test]
    fn test_set_month_as_num_passes() {
        let mut tester = get_31_jan_2023();
        let test_result = tester.set_month_num(10 /*October*/);
        assert!(test_result.is_ok());
        assert_eq!(tester.date(), date!(2023 - 10 - 31));
    }

    #[test]
    fn test_set_day_0_err() {
        let mut tester = DateTimeKeeper::now();
        let test_result = tester.set_day(0);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_day_32_err() {
        let mut tester = DateTimeKeeper::now();
        let test_result = tester.set_day(32);
        assert!(test_result.is_err());
    }
    #[test]
    fn test_set_day_february_29_no_leap_year() {
        let mut tester = DateTimeKeeper::new_from_dmy_str("28/2/2021", false).unwrap();

        let test_result = tester.set_day(29);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_day_with_0s() {
        let mut tester = get_31_jan_2023();
        let result_date = date!(2023 - 1 - 1);

        let test_result2 = tester.set_day(01);
        assert!(test_result2.is_ok());
        assert_eq!(tester.date(), result_date);

        let test_result3 = tester.set_day(001);
        assert!(test_result3.is_ok());
        assert_eq!(tester.date(), result_date);

        let test_result4 = tester.set_day(1);
        assert!(test_result4.is_ok());
        assert_eq!(tester.date(), result_date);
    }

    #[test]
    fn test_set_time_hms_none_passed() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();
        let test_result = tester.set_time_hms(None, None, None);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_time_hms_bad_hour() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();
        let test_result = tester.set_time_hms(Some(25), None, None);
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_time_hms_bad_minute() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();
        let test_result = tester.set_time_hms(None, Some(69), None);
        assert!(test_result.is_err());
    }
    #[test]
    fn test_set_time_hms_bad_second() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();
        let test_result = tester.set_time_hms(None, None, Some(69));
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_time_hms_bad_second_60() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();

        let test_result = tester.set_time_hms(None, None, Some(60));
        assert!(test_result.is_err());
    }

    #[test]
    fn test_set_time_hms_good_seconds() {
        let mut tester: DateTimeKeeper = DateTimeKeeper::new_at_midnight();
        let test_result = tester.set_time_hms(Some(0), Some(0), Some(51));
        assert!(test_result.is_ok());
        assert_eq!(tester.time(), time::Time::from_hms(0, 0, 51).unwrap())
    }

    #[test]
    fn test_apply_year_delta_forward_leap_year_february() {
        let tester = DateTimeKeeper::new_from_dmy(29, 2, 2020);
        assert!(tester.is_ok());
        let tester_keeper = tester.unwrap();
        assert_eq!(tester_keeper.date(), date!(2020 - 2 - 29));
        let result = tester_keeper.apply_year_delta(-1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().date(), date!(2019 - 2 - 28));
    }

    #[test]
    fn test_apply_year_delta_back_leap_year_february() {
        let tester = DateTimeKeeper::new_from_dmy(29, 2, 2020);
        assert!(tester.is_ok());
        let tester_keeper = tester.unwrap();
        assert_eq!(tester_keeper.date(), date!(2020 - 2 - 29));
        let result = tester_keeper.apply_year_delta(-1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().date(), date!(2019 - 2 - 28));
    }

    #[test]
    fn test_apply_year_delta_forward() {
        let tester = get_31_jan_2023();
        let year_delta_result_forward = tester.apply_year_delta(5);
        assert!(year_delta_result_forward.is_ok());
        assert_eq!(
            year_delta_result_forward.unwrap().date(),
            date!(2028 - 01 - 31)
        );
    }
    #[test]
    fn test_apply_year_delta_back() {
        let tester = get_31_jan_2023();
        let year_delta_result_back = tester.apply_year_delta(-5);
        assert_eq!(
            year_delta_result_back.unwrap().date(),
            date!(2018 - 01 - 31)
        );
    }

    #[test]
    fn test_apply_year_delta_max_forward() {
        let max_tester = DateTimeKeeper {
            utc_date_time: Date::MAX.midnight().assume_utc(),
        };

        let max_result = max_tester.apply_year_delta(1);
        assert!(max_result.is_err());
    }

    #[test]
    fn test_apply_year_delta_max_back() {
        let max_tester = DateTimeKeeper {
            utc_date_time: Date::MAX.midnight().assume_utc(),
        };
        let max_result_negative_delta = max_tester.apply_year_delta(-1);
        assert!(max_result_negative_delta.is_ok());
    }
    #[test]
    fn test_apply_year_delta_min_back() {
        let min_tester = DateTimeKeeper {
            utc_date_time: Date::MIN.midnight().assume_utc(),
        };

        let expected_fail_min_result = min_tester.apply_year_delta(-1);
        assert!(expected_fail_min_result.is_err());
    }

    #[test]
    fn test_apply_year_delta_min_forward() {
        let min_tester = DateTimeKeeper {
            utc_date_time: Date::MIN.midnight().assume_utc(),
        };
        let min_result_positive_delta = min_tester.apply_year_delta(1);
        assert!(min_result_positive_delta.is_ok());
    }

    #[test]
    fn test_apply_month_delta_forward_ok() {
        let tester = get_31_jan_2023();

        let month_delta_result_forward_ok = tester.apply_month_delta(6);
        assert!(month_delta_result_forward_ok.is_ok());
        assert_eq!(
            month_delta_result_forward_ok.unwrap().date(),
            date!(2023 - 07 - 31)
        );
    }

    #[test]
    fn test_apply_month_delta_forward_end_of_month_ok() {
        let tester = get_31_jan_2023();
        let month_delta_result_forward = tester.apply_month_delta(5);
        assert!(month_delta_result_forward.is_ok());
        assert_eq!(
            month_delta_result_forward.unwrap().date(),
            date!(2023 - 6 - 30)
        );
    }

    #[test]
    fn test_apply_month_delta_forward_1yr_exact_ok() {
        let tester = get_31_jan_2023();
        let result_forward_one_year_exact = tester.apply_month_delta(12);
        assert!(result_forward_one_year_exact.is_ok());
        assert_eq!(
            result_forward_one_year_exact.unwrap().date(),
            date!(2024 - 01 - 31)
        );
    }

    #[test]
    fn test_apply_month_delta_forward_18months_ok() {
        let tester = get_31_jan_2023();
        let result_forward_one_year_leftovers = tester.apply_month_delta(18);
        assert!(result_forward_one_year_leftovers.is_ok());
        assert_eq!(
            result_forward_one_year_leftovers.unwrap().date(),
            date!(2024 - 07 - 31)
        );
    }

    #[test]
    fn test_apply_month_delta_back_ok() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20230531", false).unwrap();
        let month_delta_result_ok = tester.apply_month_delta(-4);
        assert!(month_delta_result_ok.is_ok());
        assert_eq!(month_delta_result_ok.unwrap().date(), date!(2023 - 01 - 31));
    }

    #[test]
    fn test_apply_month_delta_back_different_day_ok() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20230531", false).unwrap();

        let month_delta_result = tester.apply_month_delta(-3);
        assert!(month_delta_result.is_ok());
        assert_eq!(month_delta_result.unwrap().date(), date!(2023 - 02 - 28));
    }

    #[test]
    fn test_apply_month_delta_back_1yr_exact_ok() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20230531", false).unwrap();
        let result_back_one_year_exact = tester.apply_month_delta(-12);
        assert!(result_back_one_year_exact.is_ok());
        assert_eq!(
            result_back_one_year_exact.unwrap().date(),
            date!(2022 - 05 - 31)
        );
    }

    #[test]
    fn test_apply_month_delta_back_16months_ok() {
        let tester = DateTimeKeeper::new_from_yyyymmdd_str("20230531", false).unwrap();
        let result_back_one_year_leftovers = tester.apply_month_delta(-16);
        assert!(result_back_one_year_leftovers.is_ok());
        assert_eq!(
            result_back_one_year_leftovers.unwrap().date(),
            date!(2022 - 01 - 31)
        );
    }

    #[test]
    fn test_apply_positive_month_delta_max_date_err() {
        let max_tester = DateTimeKeeper {
            utc_date_time: Date::MAX.midnight().assume_utc(),
        };

        let max_result: Result<DateTimeKeeper, anyhow::Error> = max_tester.apply_month_delta(1);
        assert!(max_result.is_err());
    }

    #[test]
    fn test_apply_negative_month_delta_max_date_ok() {
        let max_tester = DateTimeKeeper {
            utc_date_time: Date::MAX.midnight().assume_utc(),
        };
        let max_result_negative_delta = max_tester.apply_month_delta(-1);

        assert!(max_result_negative_delta.is_ok());
    }
    #[test]
    fn test_apply_negative_month_delta_min_date_err() {
        let min_tester = DateTimeKeeper {
            utc_date_time: Date::MIN.midnight().assume_utc(),
        };

        let expected_fail_min_result = min_tester.apply_month_delta(-1);
        assert!(expected_fail_min_result.is_err());
    }
    #[test]
    fn test_apply_positive_month_delta_min_date_ok() {
        let min_tester = DateTimeKeeper {
            utc_date_time: Date::MIN.midnight().assume_utc(),
        };

        let min_result_positive_delta = min_tester.apply_month_delta(1);

        assert!(min_result_positive_delta.is_ok());
    }

    // New tests from new_tests.rs
    #[test]
    fn test_apply_month_delta_february() {
        let tester = DateTimeKeeper::new_from_dmy(28, 2, 2023).unwrap();
        let result = tester.apply_month_delta(1).unwrap();
        assert_eq!(result.date(), date!(2023 - 3 - 28));
    }

    #[test]
    fn test_apply_month_delta_february_leap_year() {
        let tester = DateTimeKeeper::new_from_dmy(29, 2, 2020).unwrap();
        let result = tester.apply_month_delta(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().date(), date!(2020 - 3 - 29));
    }

    #[test]
    fn test_apply_month_delta_february_non_leap_year() {
        let tester = DateTimeKeeper::new_from_dmy(28, 2, 2021).unwrap();
        let result = tester.apply_month_delta(1).unwrap();
        assert_eq!(result.date(), date!(2021 - 3 - 28));
    }

    #[test]
    fn test_apply_month_delta_back_to_previous_year() {
        let tester = DateTimeKeeper::new_from_dmy(31, 1, 2023).unwrap();
        let result = tester.apply_month_delta(-1).unwrap();
        assert_eq!(result.date(), date!(2022 - 12 - 31));
    }
}
