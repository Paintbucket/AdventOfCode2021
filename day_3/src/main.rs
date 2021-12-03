use std::fs;

fn read_file(file_name : &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
    .expect("Failed to read file");

    contents.split("\n").map(|l| String::from(l.trim())).collect()
}

fn part_1() {
    let lines = read_file("input1.txt");
    let column_count = lines[0].len();
    let half_row_count = lines.len() as i32 / 2;

    let mut sums : Vec<i32> = vec!();
    sums.resize(column_count, 0);

    for l in &lines {
        let bytes = l.as_bytes();
        for i in 0..column_count {
            sums[i] += if bytes[i] == ('1' as u8) { 1 } else { 0 };
        }
    }

    let mut gamma : i32 = 0;
    let mut mask = 0;
    for i in 0..sums.len() {
        let b = sums[i] > half_row_count;
        println!("{}, {}", sums[i], if b { '1' } else {'0'});
        if b {
            gamma |= 1 << (sums.len() - i - 1);
        }
        mask |= 1 << i;
    }

    let epsilon = !gamma & mask;
    println!("gamma: {}, epsilon: {}, result: {}", gamma, epsilon, gamma * epsilon);
}

fn partition_by_column(lines: &Vec<String>, column_position: usize) -> (Vec<String>, Vec<String>) {

    let mut ones = vec!();
    let mut zeroes = vec!();

    for l in lines {
        let bytes = l.as_bytes();
        if bytes[column_position] == '1' as u8 {
            ones.push(l.clone())
        } else {
            zeroes.push(l.clone())
        }
    }

    (ones, zeroes)
}

fn get_rating(lines : &Vec<String>, pos : usize, keep_majority: bool) -> String {
    let (ones, zeroes) = partition_by_column(lines, pos);
    let to_keep;
    if keep_majority {
        to_keep = if ones.len() >= zeroes.len() { ones } else { zeroes };
    } else {
        to_keep = if ones.len() < zeroes.len() { ones } else { zeroes };
    }
    
    if to_keep.len() > 1 {
        return get_rating(&to_keep, pos + 1, keep_majority);
    } else {
        return to_keep[0].clone()
    }
        
}

fn from_binary_string(s: &str) -> i32 {
    let len = s.len();

    let mut v = 0;
    for i in 0..len {
        if s.chars().nth(i).unwrap() == '1' {
            v |= 1 << (len - i) - 1;
        }
    }

    v
}

fn part_2() {
    let lines = read_file("input1.txt");

    let oxygen_string = get_rating(&lines, 0, true);
    let oxygen_value = from_binary_string(&oxygen_string);
    println!("oxygen rating: {} - {}", oxygen_string, oxygen_value);

    let scrubber_string = get_rating(&lines, 0, false);
    let scrubber_value = from_binary_string(&scrubber_string);
    println!("scrubber rating: {} - {}", scrubber_string, scrubber_value);

    println!("Life support rating: {}", oxygen_value * scrubber_value);
}

fn main() {
    //part_1();
    part_2();
}