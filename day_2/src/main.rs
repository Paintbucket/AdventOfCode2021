use std::fs;

enum Command {
    Forward,
    Down,
    Up
}

fn read_file(file_name : &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
    .expect("Failed to read file");

    contents.split("\n").map(|l| String::from(l.trim())).collect()
}

fn map_command(s : &String) -> (Command, i32) {
    let items : Vec<&str> = s.split(' ').collect();

    let command : Command;
    match items[0] {
        "forward" => command = Command::Forward,
        "down" => command = Command::Down,
        "up" => command = Command::Up,
        _ => panic!("Unknown command!")
    }

    (command, items[1].parse().unwrap())
}

fn part_1() {
    let lines = read_file("input1.txt");

    let commands = lines.iter().map(map_command);
    let mut coords = (0, 0);
    for command in commands {
        let value = command.1;
        match command.0 {
            Command::Forward => coords.0 += value,
            Command::Down => coords.1 += value,
            Command::Up => coords.1 -= value,
        }
    }

    println!("{}", coords.0 * coords.1);
}

fn part_2() {
    let lines = read_file("input2.txt");

    let commands = lines.iter().map(map_command);
    let mut coords = (0, 0);
    let mut aim = 0;
    for command in commands {
        let value = command.1;
        match command.0 {
            Command::Forward => {
                coords.0 += value;
                coords.1 += aim * value;
            }
            Command::Down => aim += value,
            Command::Up => aim -= value,
        }
    }

    println!("{}", coords.0 * coords.1);
}

fn main() {
 
    //part_1();
    part_2()
}
