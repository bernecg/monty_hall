use monty_hall::{calculate_switch_and_stays_perc, count_switch_and_stay_wins, get_num_of_doors};

const NUM_OF_LOOPS: usize = 100000;

fn main() {
    loop {
        let num_of_doors = get_num_of_doors();

        if num_of_doors == 0 {
            return;
        }

        let (switch_wins, stay_wins) = count_switch_and_stay_wins(NUM_OF_LOOPS, num_of_doors);

        let (switch_wins_percentage, stay_wins_percentage) =
            calculate_switch_and_stays_perc(switch_wins, stay_wins, NUM_OF_LOOPS);

        println!("Switch wins: {switch_wins_percentage:.2}%",);
        println!("Stay wins: {stay_wins_percentage:.2}%");
    }
}

// codex resume 01a0c034-a351-7113-9688-707cd6d33756
