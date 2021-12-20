use std::fs;

fn parse_file(file_name: &str) -> [i32; 100] {
    let contents = fs::read_to_string(file_name).expect("Failed to read file");
    let lines = contents.lines().collect::<Vec::<&str>>();

    let mut octupii = [0i32; 100];
    for i in 0..10 {
        let line = lines[i];
        for j in 0..10 {
            let c = line.as_bytes()[j as usize] as char;
            octupii[j + i * line.len()] = c.to_digit(10).unwrap() as i32;
        }
    }

    octupii
}

fn inc(mut grid: [i32;100], x: usize, y: usize) {
    if x > 0 && x < 10 && y > 0 && y < 10 {
        grid[x + 10 * y] += 1;
    }
}

fn set(mut grid: [i32;100], x: usize, y: usize, val: i32) {
    grid[x + 10 * y] = val;
}

fn get(grid: [i32;100], x: usize, y: usize) -> i32 {
    grid[x + 10 * y]
}



fn main() {
    //let mut grid = parse_file("sample.txt");
    //let mut grid : Vec::<i32> = vec![100];
    let mut grid = [0i32;100];
    
    let mut total_flashes = 0;
    for step in 0..100 {
        let flashed = [false; 100];
        
        for y in 0..10 {
            for x in 0..10  {
                inc(grid, x, y)
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                inc(grid, x + 1, y);
                inc(grid, x - 1, y);

                inc(grid, x, y + 1);
                inc(grid, x, y - 1);

                inc(grid, x + 1, y + 1);
                inc(grid, x - 1, y - 1);

                inc(grid, x + 1, y - 1);
                inc(grid, x - 1, y + 1);
            }
        }

        let flashes = grid.iter().filter(|&&i| i > 9).count();
        println!("Flashes at step {}, total {}", flashes, total_flashes);
        total_flashes += flashes;
    }
}
