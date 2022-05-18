pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct ParseClockError;


fn main() {
    // Uncomment this!
    /*
    let clock = Clock::new(10, 10) + Clock::new(1, 50) + 4 * 60;
    println!("{}", clock);

    let clock = Clock::new(10, 10) + 1000;
    println!("{}", clock);

    let clock = Clock::new(10, 10) - 70;
    println!("{}", clock);
    */
}

/*

//
// Clock Creation
//

#[test]
fn test_on_the_hour() {
    let clock = Clock::new(8, 0);
    assert_eq!(clock.hours, 8);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_midnight_is_zero_hours() {
    let clock: Clock = Clock::new(24, 0);
    assert_eq!(clock.hours, 0);
}

#[test]
fn test_hour_rolls_over() {
    let clock = Clock::new(25, 0);
    assert_eq!(clock.hours, 1);
}

#[test]
fn test_hour_rolls_over_continuously() {
    let clock = Clock::new(100, 0);
    assert_eq!(clock.hours, 4);
}

#[test]

fn test_sixty_minutes_is_next_hour() {
    let clock = Clock::new(1, 60);
    assert_eq!(clock.hours, 2);
}

#[test]
fn test_minutes_roll_over() {
    let clock = Clock::new(0, 160);
    assert_eq!(clock.hours, 2);
    assert_eq!(clock.minutes, 40);
}

#[test]
fn test_minutes_roll_over_continuously() {
    let clock = Clock::new(0, 1723);
    assert_eq!(clock.hours, 4);
    assert_eq!(clock.minutes, 43);
}

#[test]
fn test_hours_and_minutes_roll_over() {
    let clock = Clock::new(25, 160);
    assert_eq!(clock.hours, 3);
    assert_eq!(clock.minutes, 40);
}

#[test]
fn test_hours_and_minutes_roll_over_continuously() {
    let clock = Clock::new(201, 3001);
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 1);
}

#[test]
fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
    let clock = Clock::new(72, 8640);
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_negative_hour() {
    let clock = Clock::new(-1, 15);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 15);
}

#[test]
fn test_negative_hour_roll_over() {
    let clock = Clock::new(-25, 0);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_negative_hour_roll_over_continuously() {
    let clock = Clock::new(-91, 0);
    assert_eq!(clock.hours, 5);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_negative_minutes() {
    let clock = Clock::new(1, -40);
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 20);
}

#[test]
fn test_negative_minutes_roll_over() {
    let clock = Clock::new(1, -160);
    assert_eq!(clock.hours, 22);
    assert_eq!(clock.minutes, 20);
}

#[test]
fn test_negative_minutes_roll_over_continuously() {
    let clock = Clock::new(1, -4820);
    assert_eq!(clock.hours, 16);
    assert_eq!(clock.minutes, 40);
}

#[test]
fn test_negative_sixty_minutes_is_prev_hour() {
    let clock = Clock::new(2, -60);
    assert_eq!(clock.hours, 1);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_negative_hour_and_minutes_both_roll_over() {
    let clock = Clock::new(-25, -160);
    assert_eq!(clock.hours, 20);
    assert_eq!(clock.minutes, 20);
}

#[test]
fn test_negative_hour_and_minutes_both_roll_over_continuously() {
    let clock = Clock::new(-121, -5810);
    assert_eq!(clock.hours, 22);
    assert_eq!(clock.minutes, 10);
}

#[test]
fn test_zero_hour_and_negative_minutes() {
    let clock = Clock::new(0, -22);
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 38);
}

//
// Clock Math
//

#[test]
fn test_add_minutes() {
    let clock = Clock::new(10, 0) + 3;
    assert_eq!(clock.hours, 10);
    assert_eq!(clock.minutes, 3);
}

#[test]
fn test_add_no_minutes() {
    let clock = Clock::new(6, 41) + 0;
    assert_eq!(clock.hours, 6);
    assert_eq!(clock.minutes, 41);
}

#[test]
fn test_add_to_next_hour() {
    let clock = Clock::new(0, 45) + 40;
    assert_eq!(clock.hours, 1);
    assert_eq!(clock.minutes, 25);
}

#[test]
fn test_add_more_than_one_hour() {
    let clock = Clock::new(10, 0) + 61;
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 1);
}

#[test]
fn test_add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45) + 160;
    assert_eq!(clock.hours, 3);
    assert_eq!(clock.minutes, 25);
}

#[test]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59) + 2;
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 1);
}

#[test]
fn test_add_more_than_one_day() {
    let clock = Clock::new(5, 32) + 1500;
    assert_eq!(clock.hours, 6);
    assert_eq!(clock.minutes, 32);
}

#[test]
fn test_add_more_than_two_days() {
    let clock = Clock::new(1, 1) + 3500;
    assert_eq!(clock.hours, 11);
    assert_eq!(clock.minutes, 21);
}

#[test]
fn test_subtract_minutes() {
    let clock = Clock::new(10, 3) - 3;
    assert_eq!(clock.hours, 10);
    assert_eq!(clock.minutes, 0);
}

#[test]
fn test_subtract_to_previous_hour() {
    let clock = Clock::new(10, 3) - 30;
    assert_eq!(clock.hours, 9);
    assert_eq!(clock.minutes, 33);
}

#[test]
fn test_subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3) - 70;
    assert_eq!(clock.hours, 8);
    assert_eq!(clock.minutes, 53);
}

#[test]
fn test_subtract_across_midnight() {
    let clock = Clock::new(0, 3) - 4;
    assert_eq!(clock.hours, 23);
    assert_eq!(clock.minutes, 59);
}

#[test]
fn test_subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0) - 160;
    assert_eq!(clock.hours, 21);
    assert_eq!(clock.minutes, 20);
}

#[test]
fn test_subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15) - 160;
    assert_eq!(clock.hours, 3);
    assert_eq!(clock.minutes, 35);
}

#[test]
fn test_subtract_more_than_one_day() {
    let clock = Clock::new(5, 32) - 1500;
    assert_eq!(clock.hours, 4);
    assert_eq!(clock.minutes, 32);
}

#[test]
fn test_subtract_mores_than_two_days() {
    let clock = Clock::new(2, 20) - 3000;
    assert_eq!(clock.hours, 0);
    assert_eq!(clock.minutes, 20);
}

#[test]
fn test_parse_string_into_clock() {
    let clock: Result<Clock, ParseClockError> = "12:30".parse();
    assert!(clock.is_ok());
    let clock = clock.unwrap();
    assert_eq!(clock.hours, 12);
    assert_eq!(clock.minutes, 30);
}

#[test]
fn test_parse_invalid_string_to_error() {
    let clock: Result<Clock, ParseClockError> = "1230".parse();
    assert!(clock.is_err());
}

#[test]
fn test_parse_invalid_string_with_chars_to_error() {
    let clock: Result<Clock, ParseClockError> = "aa:bb".parse();
    assert!(clock.is_err());
}

*/