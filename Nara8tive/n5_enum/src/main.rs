enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn main() {
    let go = Direction::Down;

    which_way(go);
}
