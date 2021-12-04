use std::fs;

#[derive(Debug)]
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
    Back(i32),
}

impl Command {
    fn parse(input: &str) -> Command {
        let v: Vec<&str> = input.split(' ').collect();
        match v[0] {
            "up" => Command::Up(v[1].parse().unwrap()),
            "down" => Command::Down(v[1].parse().unwrap()),
            "forward" => Command::Forward(v[1].parse().unwrap()),
            "back" => Command::Back(v[1].parse().unwrap()),
            _ => panic!("Unknown command {}", v[0]),
        }
    }
}

fn compute_position(commands: Vec<Command>) -> (i32, i32, i32) {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for command in commands {
        match command {
            Command::Up(n) => aim += n,
            Command::Down(n) => aim -= n,
            Command::Forward(n) => {
                horizontal -= n;
                depth += n * aim;
            }
            Command::Back(n) => horizontal -= n,
        };
    }
    (horizontal, depth, aim)
}

fn read_input(path: &str) -> Vec<Command> {
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = content.split('\n').collect();
    lines.iter().map(|line| Command::parse(line)).collect()
}

fn main() {
    let commands: Vec<Command> = read_input("input.txt");
    let (x, y, _aim) = compute_position(commands);

    println!("{:?}", x * y);
}
