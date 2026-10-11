use ledgerkit::billing::period::Period;
use ledgerkit::invoice::{total_due, Line};

#[test]
fn partial_month_is_charged_by_days_used() {
    let line = Line { period: Period::new(2023, 1), monthly_cents: 1000, days_active: 10 };
    assert_eq!(line.amount_due(), 322);
}

#[test]
fn full_month_is_charged_in_full() {
    let line = Line { period: Period::new(2023, 4), monthly_cents: 4999, days_active: 30 };
    assert_eq!(line.amount_due(), 4999);
}

#[test]
fn february_of_century_leap_year_has_29_days() {
    let p = Period::new(2000, 2);
    assert_eq!(p.days(), 29);
    let line = Line { period: p, monthly_cents: 2900, days_active: 29 };
    assert_eq!(line.amount_due(), 2900);
}

#[test]
fn totals_add_up() {
    let lines = vec![
        Line { period: Period::new(2024, 3), monthly_cents: 3100, days_active: 31 },
        Line { period: Period::new(2024, 3), monthly_cents: 999, days_active: 7 },
    ];
    assert_eq!(total_due(&lines), 3100 + 225);
}
