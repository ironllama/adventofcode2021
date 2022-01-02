use std::collections::HashMap;

fn main() {
    // let starting_1 = 4;
    // let starting_2 = 8;
    let starting_1 = 8;
    let starting_2 = 4;

    const MAX_SCORE: i64 = 21;

    // We could probably save time with the rolls, because with a given dice and sides, there's only so many combos.
    const DICE_COMBOS: [(i64, i64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    fn turn(player_idx: usize, players: &Vec<Vec<i64>>, all_states: &mut HashMap<(usize, Vec<Vec<i64>>), Vec<i64>>) -> Vec<i64> {
        if players[0][1] >= MAX_SCORE { return vec![1, 0]; }
        if players[1][1] >= MAX_SCORE { return vec![0, 1]; }

        // let player_idx = turn_num % 2;
        let mut wins: Vec<i64> = vec![0, 0];
        for &(roll, num_poss) in DICE_COMBOS.iter() {
            let mut new_players: Vec<Vec<i64>> = players.clone();
            new_players[player_idx][0] = (new_players[player_idx][0] + roll) % 10;  // Assign new board space.
            new_players[player_idx][1] += new_players[player_idx][0] + 1;  // Points: 0-9 range of the board converted to 1-10 points
            let new_player_idx = if player_idx == 1 {0} else {1};

            let state_key: (usize, Vec<Vec<i64>>) = (new_player_idx, new_players.to_owned());  // UGLY. Wedging in the new_player_idx to make it work correctly.
            let new_wins: Vec<i64> = match all_states.get(&state_key) {  // Use cache, if exists. Or insert if not.
                Some(x) => x.to_owned(),
                None => {
                    let recur_wins: Vec<i64> = turn(new_player_idx, &new_players, all_states);
                    all_states.insert(state_key, recur_wins.to_owned());  // Maybe use tuples instead of vectors to cleanup clone() and to_owned()?
                    recur_wins
                }
            };

            wins[0] += num_poss * new_wins[0];  // Don't know which wins, so mutliply universes (0 or 1) with both win indices.
            wins[1] += num_poss * new_wins[1];
        }
        wins
    }

    let players: Vec<Vec<i64>> = vec![vec![starting_1 - 1, 0], vec![starting_2 - 1, 0]]; // Outer Vec is two players. Each player has Vec of position and score.
    let wins: Vec<i64> = turn(0, &players, &mut HashMap::new());

    println!("WINS: {:?}", wins.iter().max().unwrap());
}


    // From tweaking the possible moves, looks like there is at most 10 - 11 turns? Usable?
    // // Roll the dice to get 3 combined rolls. Returns the total of the 3 rolls.
    // // Could have also used a generator for each of the two dice, as they return a predictable pattern of results.
    // // Generators are still RFC in Rust? And closures still have memory copy overhead.
    // // fn roll_dice(main_dice: &mut i64) -> Vec<i64> {
    // //     let mut dice_results = vec![*main_dice, ((*main_dice + 1) % 3), ((*main_dice + 2) % 3)];
    // //     dice_results.sort();
    // //     *main_dice = (*main_dice + 3) % 3;
    // //     dice_results
    // // }

    // let mut players: Vec<Vec<i64>> = vec![vec![starting_1 - 1, 0], vec![starting_2 - 1, 0]];
    // let mut main_dice: i64 = 1;
    // let mut turn = 1;
    // let mut dice_rolls = 1;
    // // Spaces are tracked as 0-9 for easy use of modulo. Tradeoff is some complexity when to convert it to 1-10.
    // while players[0][1] < MAX_SCORE && players[1][1] < MAX_SCORE {
    //     // let next_roll = roll_dice(&mut main_dice)[0];
    //     // let next_roll = 3;
    //     // let next_roll = 1;
    //     dice_rolls *= 3;

    //     // Rewrote above with a flexible reference. Academic -- just wanted to try and learn. Probably not more readable.
    //     let mut this_player: &mut Vec<i64> = &mut players[0];  // The explicit type was not necessary, but helped debug!
    //     if turn %2 == 0 {
    //         this_player = &mut players[1];
    //     }

    //     // Let smallest possible score -- trying to get most number of moves.
    //     let mut new_move = 9;
    //     for next_roll in 1..=3 {
    //         let possible_move = this_player[0] + next_roll % 10;
    //         if possible_move < new_move {
    //             new_move = possible_move;
    //         }
    //     }
    //     this_player[0] = (this_player[0] + new_move) % 10;
    //     this_player[1] += this_player[0] + 1;  // 0-9 range converted to 1-10
    //     println!("PLAYER {}: MOVE {} BOARD {} SCORE {}", if turn %2 == 1 {'1'} else {'2'}, new_move, this_player[0] + 1, this_player[1]);

    //     turn += 1;
    // }

    // // Display.
    // println!("TURNS: {}", turn);
    // for (i, player) in players.iter().enumerate() {
    //     println!("PLAYER {}: NUM_ROLLS {} BOARD {} SCORE {}", i+1, dice_rolls, player[0] + 1, player[1]);
    // }

    // // let losing_score = std::cmp::min(players[0][1], players[1][1]);
    // // println!("END: {}", losing_score * dice_rolls);
// }