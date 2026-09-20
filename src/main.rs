use rand::RngExt;
use monty_hall::{get_switched_door, set_monty_door};

fn main() {
    let mut rng = rand::rng();
    let mut switch_wins = 0;
    let mut stay_wins = 0;
    let num_of_loops = 100000;

    for _ in 0..num_of_loops {
        let prize_door = rng.random_range(0..3);
        let initial_choice = rng.random_range(0..3);
        let monty_door = set_monty_door(prize_door, initial_choice);
        let switched_door = get_switched_door(initial_choice, monty_door);

        if switched_door == prize_door {
            switch_wins += 1;
        }
        if initial_choice == prize_door {
            stay_wins += 1;
        }
    }

    let switch_wins_percentage = switch_wins as f64 / num_of_loops as f64 * 100.0;
    let stay_wins_percentage = stay_wins as f64 / num_of_loops as f64 * 100.0;

    println!("Switch wins: {switch_wins_percentage:.2}%",);
    println!("Stay wins: {stay_wins_percentage:.2}%");
}
