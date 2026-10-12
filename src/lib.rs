//! A Monte Carlo simulation of the Monty Hall problem.
//!
//! The simulation compares two strategies:
//!
//! - staying with the player's initial choice;
//! - switching to the other door left available by Monty.
//!
//! With three or more doors, switching wins whenever the initial choice was
//! wrong, so the simulation should show switching winning more often.

use rand::RngExt;
use std::io;

/// Asks the user for the number of doors.
///
/// Valid game values are numbers greater than or equal to `3`. Entering `0`
/// tells the application to stop. Invalid input is rejected and the user is
/// asked again.
pub fn get_num_of_doors() -> usize {
    loop {
        println!("Enter the number of doors (0 ends the game):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(number) if number >= 3 || number == 0 => return number,
            _ => {
                println!("Please enter a number of doors equal or greater to 3 (0 ends the game).")
            }
        };
    }
}

/// Runs a Monty Hall simulation for the requested number of rounds.
///
/// Returns `(switch_wins, stay_wins)`. Each round randomly chooses a prize
/// door and an initial choice. Switching wins when those two doors differ;
/// staying wins when they are the same.
///
/// # Panics
///
/// This function panics if `num_of_doors` is `0` and `num_of_loops` is greater
/// than `0`, because there are no valid doors to choose from. The interactive
/// input function prevents this situation by accepting only `0` or values of
/// at least `3`.
pub fn count_switch_and_stay_wins(num_of_loops: usize, num_of_doors: usize) -> (usize, usize) {
    let mut rng = rand::rng();
    let mut stay_wins = 0;
    let mut switch_wins = 0;

    for _ in 0..num_of_loops {
        let prize_door = rng.random_range(1..num_of_doors + 1);
        let initial_choice = rng.random_range(1..num_of_doors + 1);

        if initial_choice == prize_door {
            stay_wins += 1;
        } else {
            switch_wins += 1;
        }
    }
    (switch_wins, stay_wins)
}

/// Converts win counts into percentages.
///
/// Returns `(switch_percentage, stay_percentage)`.
///
/// # Examples
///
/// ```
/// let percentages = monty_hall::calculate_switch_and_stays_perc(80, 20, 100);
/// assert_eq!(percentages, (80.0, 20.0));
/// ```
pub fn calculate_switch_and_stays_perc(
    switch_wins: usize,
    stay_wins: usize,
    num_of_loops: usize,
) -> (f64, f64) {
    if num_of_loops == 0 {
        return (0.0, 0.0);
    }

    let switch_wins_percentage = switch_wins as f64 / num_of_loops as f64 * 100.0;
    let stay_wins_percentage = stay_wins as f64 / num_of_loops as f64 * 100.0;

    (switch_wins_percentage, stay_wins_percentage)
}
