use monty_hall::get_switched_door;

#[test]
fn switched_door_is_the_remaining_door() {
    assert_eq!(get_switched_door(0, 2), 1);
}