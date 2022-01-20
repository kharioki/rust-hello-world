enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Player is moving up"),
        Direction::Down => println!("Player is moving down"),
        Direction::Left => println!("Player is moving left"),
        Direction::Right => println!("Player is moving right"),
    }
}
