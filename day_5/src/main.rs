use std::fs;

fn parse_file(file_name : &str) {
    let contents = fs::read_to_string(file_name).expect("Failed to read file");
    
    for line in contents.lines() {
        let parts : Vec::<&str> = line.split(" -> ").collect();
        let mut first_tuple = parts[0].split(",");
        let a : i32 = first_tuple.next().unwrap().parse().unwrap();
        let b : i32 = first_tuple.next().unwrap().parse().unwrap();

        let mut second_tuple = parts[1].split(",");
        let c : i32 = second_tuple.next().unwrap().parse().unwrap();
        let d : i32 = second_tuple.next().unwrap().parse().unwrap();

        println!("{}, {} => {}, {}", a, b, c, d);
    }
}

fn main() {
    parse_file("sample.txt");
}
