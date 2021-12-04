struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn mov(pos: &mut Position, command: &str) {
    let parts: Vec<&str> = command.split(' ').collect();
    let direction = parts[0];
    let distance = parts[1].parse::<i32>().unwrap();

    match direction {
        "forward" => {
            pos.horizontal += distance;
            pos.depth += pos.aim * distance;
        },
        "down" => {
            pos.aim += distance;
        },
        "up" => {
            pos.aim -= distance;
        },
        _ => println!("Error"),
    }
}

pub fn go2(commands: Vec<String>) {
    let mut pos = Position::new();
    for command in commands {
        mov(&mut pos, &command);
    }
    println!("Final position: {} {}", pos.horizontal, pos.depth);
}
