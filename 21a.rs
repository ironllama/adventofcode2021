fn main() {
    // let starting_1 = 4;
    // let starting_2 = 8;
    let starting_1 = 8;
    let starting_2 = 4;

    // let max_score = 100;
    let max_score = 1000;

    // Roll the dice to get 3 combined rolls. Returns the total of the 3 rolls.
    // Could have also used a generator for each of the two dice, as they return a predictable pattern of results.
    fn roll_dice(main_dice: &mut i32) -> i32 {
        let dice_total = ((*main_dice) % 100) + ((*main_dice + 1) % 100) + ((*main_dice + 2) % 100);
        *main_dice = (*main_dice + 3) % 100;
        dice_total
    }

    let mut players: Vec<Vec<i32>> = vec![vec![starting_1 - 1, 0], vec![starting_2 - 1, 0]];
    let mut main_dice: i32 = 1;
    let mut turn = 1;
    let mut dice_rolls = 0;
    // Spaces are tracked as 0-9 for easy use of modulo. Tradeoff is some complexity when to convert it to 1-10.
    while players[0][1] < max_score && players[1][1] < max_score {
        let next_roll = roll_dice(&mut main_dice);
        dice_rolls += 3;

        // Original, with explicitly addressing the two players.
        // if turn % 2 == 1 {
        //     let new_space = players[0][0] + next_roll;
        //     players[0][0] = new_space % 10;
        //     players[0][1] += players[0][0] + 1;  // 0-9 range converted to 1-10
        //     // println!("PLAYER {}: ROLL {} BOARD {} SCORE {}", if turn %2 == 1 {'1'} else {'2'}, next_roll, players[0][0] + 1, players[0][1]);
        // }
        // else {
        //     let new_space = players[1][0] + next_roll;
        //     players[1][0] = new_space % 10;
        //     players[1][1] += players[1][0] + 1;  // 0-9 range converted to 1-10
        //     // println!("PLAYER {}: ROLL {} BOARD {} SCORE {}", if turn %2 == 1 {'1'} else {'2'}, next_roll, players[1][0] + 1, players[1][1]);
        // }

        // Rewrote above with a flexible reference. Academic -- just wanted to try and learn. Probably not more readable.
        let mut this_player: &mut Vec<i32> = &mut players[0];  // The explicit type was not necessary, but helped debug!
        if turn %2 == 0 {
            this_player = &mut players[1];
        }

        let new_space = this_player[0] + next_roll;
        this_player[0] = new_space % 10;
        this_player[1] += this_player[0] + 1;  // 0-9 range converted to 1-10
        // println!("PLAYER {}: ROLL {} BOARD {} SCORE {}", if turn %2 == 1 {'1'} else {'2'}, next_roll, this_player[0] + 1, this_player[1]);

        turn += 1;
    }

    // Display.
    // for (i, player) in players.iter().enumerate() {
    //     println!("PLAYER {}: NUM_ROLLS {} BOARD {} SCORE {}", i+1, dice_rolls, player[0] + 1, player[1]);
    // }

    let losing_score = std::cmp::min(players[0][1], players[1][1]);
    println!("END: {}", losing_score * dice_rolls);
}