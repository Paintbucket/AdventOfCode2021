use std::fs;

fn read_file(file_name : &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
    .expect("Failed to read file");

    contents.split("\n").map(|l| String::from(l.trim())).collect()
}

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<(i32, bool)>>
}

impl Board {
    fn bingo(self: &Self) -> bool {
        for r in &self.rows {
            if r.iter().all(|v| v.1) {
                return true
            }
        }

        for i in 0..5 {
            let column = vec![self.rows[0][i], self.rows[1][i], self.rows[2][i], self.rows[3][i], self.rows[4][i]];
            if column.iter().all(|v| v.1) {
                return true
            }
        }

        false
    }

    fn mark(self: &mut Self, v : i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i][j].0 == v {
                    self.rows[i][j] = (v, true);
                }
            }
        }
    }

    fn sum(self: &Self) -> i32 {
        let mut sum = 0;
        for r in &self.rows {
            sum = r.iter().filter(|p| !p.1).fold(sum, |s, pair| s + pair.0)
        }
        sum
    }
}

fn print_board(b: &Board) {
    println!("Board:");
    for r in &b.rows {
        for p in r {
            print!("{}{},", p.0, if p.1 { "X" } else { "_"});
        }
        println!();
    }
}

fn parse_board(lines: &[String]) -> Board {
    let mut rows = vec!();
    for l in lines {
        let r = l.split(' ').filter(|c| c.len() > 0).map(|c| (c.trim().parse().unwrap(), false)).collect();
        rows.push(r);
    }
    Board {
        rows : rows
    }
}


fn parse_boards(lines: &[String]) -> Vec<Board> {
    let mut boards : Vec<Board> = vec!();
    for i in (0..lines.len()).step_by(6) {
        boards.push(parse_board(&lines[i..i+5]));
    }
    boards
}

fn part_1(numbers: Vec<i32>, mut boards: Vec<Board>) {
    for n in numbers {
        for b in &mut boards {
            b.mark(n)
        }

        for b in &boards {
            if b.bingo() {
                let sum = b.sum();
                println!("Winner! sum: {}, number: {}, total: {}", sum, n, sum * n);
                print_board(&b);
                return;
            }
        }
    }
}

fn part_2(numbers: Vec<i32>, mut boards: Vec<Board>) {
    let mut remaining_boards = boards.clone();

    for n in numbers {
        for b in &mut remaining_boards {
            b.mark(n)
        }

        if (remaining_boards.len() == 1 && remaining_boards[0].bingo()) {

            println!("Last board. {}, {}, {}", remaining_boards[0].sum(), n, remaining_boards[0].sum() * n);
            print_board(&remaining_boards[0]);
            return;
        }

        remaining_boards = remaining_boards.into_iter().filter(|b| !b.bingo()).collect();
    }
}

fn main() {
    let lines = read_file("input.txt");
    let numbers : Vec<i32> = lines[0].split(',').map(|c| c.parse().unwrap()).collect();
    let mut boards = parse_boards(&lines[2..]);
    
    //part_1(numbers.clone(), boards.clone());
    part_2(numbers.clone(), boards.clone());
}
