use rand::RngExt;
use std::io;

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
