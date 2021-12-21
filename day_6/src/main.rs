
fn print_fish(fish: &[i32]) {
    for n in fish {
        print!("{},", n)
    }
    println!();
}

fn part_1(input: &Vec::<i32>) {
    let mut fish = input.clone();

    print!("Initial state: ");
    print_fish(&fish);

    for day in 0..80 {
        let mut added = vec!();
        for n in fish.iter_mut() {
            *n = *n - 1;
            if *n < 0 {
                *n = 6;
                added.push(8);
            }
        }

        for new in added {
            fish.push(new)
        }
    }

    println!("Fish count: {}", fish.len());
}

fn print_fish_per_age(fish: &[i64]) {
    for f in fish {
        print!("{},", f);
    }
    println!();
}

fn part_2(input: &Vec::<i64>, days: i32) {
    let mut fish = input.clone();

    let mut fish_per_age: Vec::<i64> = (0..=8).map(|v| 0).collect();

    for f in fish {
        fish_per_age[f as usize] += 1
    }

    print!("Initial state: ");
    print_fish_per_age(&fish_per_age);

    for day in 0..days {
        fish_per_age.rotate_left(1);
        fish_per_age[6] += fish_per_age[8];

        print!("Day {}: ", day + 1);
        print_fish_per_age(&fish_per_age)
    }

    println!("Fish count: {}", fish_per_age.iter().sum::<i64>());
}

fn main() {

    let sample = vec![3,4,3,1,2];
    let input : Vec::<i64> = vec![4,1,1,1,5,1,3,1,5,3,4,3,3,1,3,3,1,5,3,2,4,4,3,4,1,4,2,2,1,3,5,1,1,3,2,5,1,1,4,2,5,4,3,2,5,3,3,4,5,4,3,5,4,2,5,5,2,2,2,3,5,5,4,2,1,1,5,1,4,3,2,2,1,2,1,5,3,3,3,5,1,5,4,2,2,2,1,4,2,5,2,3,3,2,3,4,4,1,4,4,3,1,1,1,1,1,4,4,5,4,2,5,1,5,4,4,5,2,3,5,4,1,4,5,2,1,1,2,5,4,5,5,1,1,1,1,1,4,5,3,1,3,4,3,3,1,5,4,2,1,4,4,4,1,1,3,1,3,5,3,1,4,5,3,5,1,1,2,2,4,4,1,4,1,3,1,1,3,1,3,3,5,4,2,1,1,2,1,2,3,3,5,4,1,1,2,1,2,5,3,1,5,4,3,1,5,2,3,4,4,3,1,1,1,2,1,1,2,1,5,4,2,2,1,4,3,1,1,1,1,3,1,5,2,4,1,3,2,3,4,3,4,2,1,2,1,2,4,2,1,5,2,2,5,5,1,1,2,3,1,1,1,3,5,1,3,5,1,3,3,2,4,5,5,3,1,4,1,5,2,4,5,5,5,2,4,2,2,5,2,4,1,3,2,1,1,4,4,1,5];

    part_2(&input, 256);

    
}
