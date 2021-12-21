
fn advance_board(pos: &mut i32, n: i32) {
    *pos = (*pos + n) % 10;
    if *pos == 0 { *pos = 10 }
}

fn main() {

    let _sample_start = (4, 8);
    let input_start = (8, 6);

    let mut player_positions = input_start;
    let mut player_scores = (0, 0);

    let mut roll_count = 0;
    let mut die = 0;
    let mut dice = std::iter::from_fn(move || {
        die = ((die + 1) % 101).max(1);
        Some(die)
    });

    let losing_score;

    loop {
        
        {
            let sum: i32 = [dice.next().unwrap(), dice.next().unwrap(), dice.next().unwrap()].iter().sum();
            roll_count += 3;
        
            let last_pos = player_positions.0;
            advance_board(&mut player_positions.0, sum);

            player_scores.0 += player_positions.0;

            println!("Player 1 rolls {}, from position = {}, to position = {}, score = {}", sum, last_pos, player_positions.0, player_scores.0);

            if player_scores.0 >= 1000 {
                losing_score = player_scores.1;
                break;
            }
        }

        {
            let sum: i32 = [dice.next().unwrap(), dice.next().unwrap(), dice.next().unwrap()].iter().sum();
            roll_count += 3;

            let last_pos = player_positions.1;
            advance_board(&mut player_positions.1, sum);

            player_scores.1 += player_positions.1;

            println!("Player 2 rolls {}, from position = {}, to position = {}, score = {}", sum, last_pos, player_positions.1, player_scores.0);

            if player_scores.1 >= 1000 {
                losing_score = player_scores.0;
                break;
            }
        }
    }

    println!("Player one score {}", player_scores.0);
    println!("Player two score {}", player_scores.1);
    println!("Product {} x {} = {}", roll_count, losing_score, roll_count * losing_score)
}
