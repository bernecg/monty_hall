pub fn set_monty_door(prize_door: i32, initial_choice: i32) -> i32 {
    let mut monty_door = 0;

    if monty_door == prize_door || monty_door == initial_choice {
        monty_door = 1;
    }
    if monty_door == prize_door || monty_door == initial_choice {
        monty_door = 2;
    }

    monty_door
}

pub fn get_switched_door(initial_choice: i32, monty_door: i32) -> i32 {
    let mut switched_door = -1;
    if initial_choice != 0 && monty_door != 0 {
        switched_door = 0;
    } else if initial_choice != 1 && monty_door != 1 {
        switched_door = 1;
    } else if initial_choice != 2 && monty_door != 2 {
        switched_door = 2;
    }

    switched_door
}