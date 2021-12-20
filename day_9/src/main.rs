use std::fs;

fn parse_file(file_name: &str) -> (Vec::<i32>, i32, i32) {
    let contents = fs::read_to_string(file_name).expect("Failed to read file");

    let mut output = vec!();
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    for line in contents.lines() {
        let mut numbers : Vec::<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        cols = numbers.len() as i32;
        rows += 1;
        output.append(&mut numbers);
    }

    (output, rows, cols)
}

struct Neighbors {
    top: Option<i32>,
    bottom: Option<i32>,
    left: Option<i32>,
    right: Option<i32>,
}

type Grid = (Vec::<i32>, i32, i32);

fn get(grid: &Grid, x: i32, y: i32) -> i32 {
    grid.0[(x + grid.2 *y) as usize]
}

fn neighbors(grid: &Grid, x: i32, y: i32) -> Neighbors {
    let row_count = grid.1;
    let column_count = grid.2;

    Neighbors {
        top: if y - 1 >= 0 { Some(get(&grid, x, y - 1))} else {None},
        bottom: if y + 1 < row_count { Some(get(&grid, x, y + 1))} else {None},
        left: if x - 1 >= 0 { Some(get(&grid, x - 1, y))} else {None},
        right: if x + 1 < column_count { Some(get(&grid, x + 1, y))} else {None},
    }
}

fn is_low_point(grid: &Grid, x: i32, y: i32) -> bool {
    let me = get(&grid, x, y);
    let neighbors = neighbors(grid, x, y);
    return [neighbors.top, neighbors.bottom, neighbors.left, neighbors.right].iter().filter(|n| n.is_some()).fold(true, |curr, o| o.unwrap() > me && curr );
}

fn collect_basin(grid: &Grid, x: i32, y: i32, previous_value: i32, basin: &mut Vec::<i32>, visited: &mut Vec::<(i32, i32)>) {

    let has_already_visited = visited.iter().any(|(vx, vy)| x == *vx && y == *vy);
    if has_already_visited {
        return;
    }

    let this_value = get(&grid, x, y);
    if this_value < previous_value || this_value >= 9 {
        return;
    }

    visited.push((x, y));
    basin.push(this_value);

    let ns = neighbors(grid, x, y);
    
    if ns.top.is_some() {
        collect_basin(grid, x, y - 1, this_value, basin, visited);
    }
    if ns.bottom.is_some() {
        collect_basin(grid, x, y + 1, this_value, basin, visited);
    }
    if ns.left.is_some() {
        collect_basin(grid, x - 1, y, this_value, basin, visited);
    }
    if ns.right.is_some() {
        collect_basin(grid, x + 1, y, this_value, basin, visited);
    }
}

fn main() {
    let grid = parse_file("input.txt");

    let mut risk_level_sum = 0;
    let mut basin_sizes : Vec::<i32> = vec!();
    for y in 0..grid.1 {
        for x in 0..grid.2 {
            if is_low_point(&grid, x, y) {
                let value = get(&grid, x, y);
                risk_level_sum += value + 1;

                let mut visited_nodes = vec!();
                let mut basin = vec!();
                collect_basin(&grid, x, y, value, &mut basin, &mut visited_nodes);
                basin_sizes.push(basin.len() as i32);

                println!("Low point ({}, {}) has a basin (size {}) including ", x, y, basin.len());
                for b in basin {
                    print!("{}, ", b);
                }
                println!();
            }
        }
    }

    basin_sizes.sort();
    let three_largest_basins: &[i32] = &basin_sizes[basin_sizes.len()-3..];
    let basin_product = three_largest_basins.iter().fold(1, |p, v| v * p);

    println!("Total risk level: {}", risk_level_sum);
    println!("Basin product: {}", basin_product);
}
