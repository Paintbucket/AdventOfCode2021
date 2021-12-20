use std::fs;

type SignalsAndOutputs = Vec::<(Vec::<String>, Vec::<String>)>;

fn parse_file(file_name: &str) -> SignalsAndOutputs {
    let contents = fs::read_to_string(file_name).expect("Failed to read file");

    let mut output : SignalsAndOutputs = vec!();
    for line in contents.lines() {
        let parts : Vec::<&str> = line.split(" | ").collect();
        let signals: Vec::<String> = parts[0].split(" ").map(|s| String::from(s)).collect();
        let outputs: Vec::<String> = parts[1].split(" ").map(|s| String::from(s)).collect();
        output.push((signals.clone(), outputs));
    }

    output
}

fn part_1(signals : &SignalsAndOutputs) {
    let count = signals.iter()
    .fold(0, |sum, outputs| 
        sum + outputs.1.iter().filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7).count()
    );
    
    println!("1, 4, 7, or 8 appears {} times", count);
}

fn find_number(c: char, signals: &Vec<String>) -> String {
    match c {
        '1' => String::from(signals.iter().filter(|s| s.len() == 2).next().unwrap()),
        '4' => String::from(signals.iter().filter(|s| s.len() == 4).next().unwrap()),
        '7' => String::from(signals.iter().filter(|s| s.len() == 3).next().unwrap()),
        '8' => String::from(signals.iter().filter(|s| s.len() == 7).next().unwrap()),
        _ => String::from("UNKNOWN")
    }
}

fn part_2(signals: &SignalsAndOutputs) {
    for p in signals {
        for signal in &p.0
        {
            let ss : &str = &signal;
            let mut v : Vec::<char> = ss.chars().collect();
            v.sort();
            print!("{}", String::from_iter(v));
            print!(" ");
        }
        let (one, four, seven, eight) = (find_number('1', &p.0), find_number('4', &p.0), find_number('7', &p.0), find_number('8', &p.0) );
        println!();
        for c in '0'..'9' {
            print!("{} = {}, ", c, find_number(c, &p.0));
        }
        
        println!();
        
    }
}

fn main() {
    let input = parse_file("sample.txt");
    part_1(&input);
    part_2(&input);
}
