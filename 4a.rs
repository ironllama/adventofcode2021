pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
    //     "",
    //     "22 13 17 11  0",
    //     "8  2 23  4 24",
    //     "21  9 14 16  7",
    //     "6 10  3 18  5",
    //     "1 12 20 15 19",
    //     "",
    //     "3 15  0  2 22",
    //     "9 18 13 17  5",
    //     "19  8  7 25 23",
    //     "20 11 10 24  4",
    //     "14 21 16 12  6",
    //     "",
    //     "14 21 17 24  4",
    //     "10 16 15  9 19",
    //     "18  8 23 26 20",
    //     "22 11 13  6  5",
    //     "2  0 12  3  7",
    //     // "",
    //     // "14 21 7 24  4",
    //     // "10 16 5  9 19",
    //     // "18  8 4 26 20",
    //     // "22 11 9  6  5",
    //     // "2  0 11  3  7",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("4.in.txt");

    // Function that returns if/when a specific row will win.
    fn check_row(in_row: &Vec<&str>, called_nums: &Vec<&str>) -> i32 {
        let mut num_matches = 0;
        let mut last_match = 0;
        for (idx, num) in called_nums.iter().enumerate() {
            if in_row.contains(num) {
                num_matches += 1;
                if num_matches == in_row.len() {
                    last_match = idx as i32;
                    break;
                }
            }
        }
        return last_match;
    }

    // Preprocess all the boards into a Vector of boards
    let called_nums: Vec<&str> = input_stuff[0].split(',').collect();
    let boards: Vec<Vec<Vec<Vec<&str>>>> = input_stuff.iter().enumerate().fold(vec![], | mut acc, (i, line) | {
        if i < 1 { return acc; }

        if line == &"" {  // Using blank space as separator between populating boards.
            let new_vec = vec![vec![], vec![]];
            acc.push(new_vec);
            // println!("ACC NEW: {:?}", acc);
        }
        else {
            let split_line: Vec<&str> = line.split_whitespace().collect();  // Split line into vector.
            let last = acc.last_mut().unwrap();

            // Add regular board.
            last[0].push(split_line.clone());
            
            // Add rotated board.
            if last[1].len() > 0 {
                for num in 0..split_line.len() {
                    last[1][num].push(split_line[num]);
                }
            }
            else {
                for num in split_line.iter() {
                    last[1].push(vec![num]);
                }
            }
        }

        return acc;
    });
    // println!("{:?}", boards);

    // Find winning board.
    let mut winning_idx = 0;
    let mut winning_num = 0;
    let mut winning_row: &Vec<&str> = &vec![];
    let mut winning_type: i32 = 0;
    let mut winning_board: &Vec<Vec<&str>> = &vec![];
    for (board_idx, board) in boards.iter().enumerate() {
        for (board_type_idx, board_type) in board.iter().enumerate() {
            for row in board_type.iter() {
                let num_to_win = check_row(row, &called_nums);
                // println!("BOARD: {} ROW: {:?} NUM: {}", board_idx, row, num_to_win);
                if num_to_win >=5 && (winning_idx == 0 || num_to_win < winning_idx) {
                    winning_idx = num_to_win;
                    winning_num = called_nums[winning_idx as usize].parse::<i32>().unwrap();
                    winning_row = &row;
                    winning_type = board_type_idx as i32;
                    winning_board = &board_type;
                }
            }
        }
    }
    // println!("FINAL: BOARD: {:?} TYPE: {} ROW: {:?} NUM: {} IDX: {}", winning_board, winning_type, winning_row, winning_num, winning_idx);

    // Flatten wininng board for easier scoring.
    let (called_nums_until_win, _) = called_nums.split_at(winning_idx as usize + 1);
    // println!("CALLED: {:?}", called_nums_until_win);
    let mut left_overs = winning_board.iter().fold(vec![], | mut acc, line | {
        for num in line {
            let num_int = num.parse::<i32>().unwrap();
            acc.push(num_int);
        }
        acc
        // acc.extend(line);
        // acc
    });
    // let flattened_board_int: Vec<i32> = flattened_board.iter().map(|num_str| num_str.parse::<i32>().unwrap()).collect();
    // println!("FLAT ALL: {:?}", left_overs);

    // Remove called numbers.
    for num_str in called_nums_until_win.iter() {
        let num_int = num_str.parse::<i32>().unwrap();
        left_overs.retain(|&x| x != num_int);
    }

    // Scoring.
    let total_uncalled = left_overs.iter().sum::<i32>();
    // println!("FLAT REMAINING: {:?}", left_overs);
    // println!("UNCALLED SUM: {}", total_uncalled);

    println!("SCORE: {}", total_uncalled * winning_num);
}