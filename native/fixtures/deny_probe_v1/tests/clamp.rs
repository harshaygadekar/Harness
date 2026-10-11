use deny_probe::clamp::clamp;

#[test]
fn inside_range_unchanged() {
    assert_eq!(clamp(5, 0, 10), 5);
}

#[test]
fn below_range_goes_to_lo() {
    assert_eq!(clamp(-3, 0, 10), 0);
}

#[test]
fn above_range_goes_to_hi() {
    assert_eq!(clamp(42, 0, 10), 10);
}
