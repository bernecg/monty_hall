use monty_hall::{calculate_switch_and_stays_perc, count_switch_and_stay_wins};

#[test]
fn win_counts_add_up_to_number_of_loops() {
    let (switch_wins, stay_wins) = count_switch_and_stay_wins(10000, 10);
    assert_eq!(switch_wins + stay_wins, 10000);
}

#[test]
fn zero_loops_produce_zero_wins() {
    assert_eq!(count_switch_and_stay_wins(0, 3), (0, 0));
}

#[test]
fn percentages_add_up_to_one_hundred() {
    let (switch_percentage, stay_percentage) = calculate_switch_and_stays_perc(70, 30, 100);
    assert_eq!(switch_percentage + stay_percentage, 100.0);
}

#[test]
fn verify_percentages_switch_and_stays() {
    assert_eq!(calculate_switch_and_stays_perc(8, 7, 10), (80.0, 70.0));
}

#[test]
fn verify_percentages_zero_loops() {
    assert_eq!(calculate_switch_and_stays_perc(8, 7, 0), (0.0, 0.0));
}
