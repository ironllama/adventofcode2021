pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "1163751742",
    //     "1381373672",
    //     "2136511328",
    //     "3694931569",
    //     "7463417111",
    //     "1319128137",
    //     "1359912421",
    //     "3125421639",
    //     "1293138521",
    //     "2311944581",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("15.in.txt");

    // Get the input data the way I want to use it -- vector of vector of integers.
    let input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });
    // println!("{:?}", input_stuff_vec);

    // Create the tapestry from the input, each "block" one higher than the last.
    let mut input_stuff_matrix: Vec<Vec<Vec<Vec<i32>>>> = vec![];
    for row in 0..5 {
        let mut row_input = vec![];
        for col in row..(row + 5) {  // Each row starts with values one higher than the last row
            let mut col_input = vec![];
            input_stuff_vec.iter().for_each(|line| {
                let next_line: Vec<i32> = line.iter().map(|x| {
                    let mut new_num = x + col;
                    if new_num > 9 {
                        new_num = new_num - 9;
                    }
                    new_num
                }).collect();
                col_input.push(next_line);
            });
            row_input.push(col_input);
        }
        input_stuff_matrix.push(row_input);
    }
    // Display.
    // input_stuff_matrix.iter().for_each(|line| {
    //     for i in 0..line[0].len() {
    //         for k in 0..line.len() {
    //             line[k][i].iter().for_each(|x| print!("{}", x));
    //             print!(" ");
    //         }
    //         println!();
    //     }
    //     println!();
    // });

    // Flatten the input_stuff_matrix, merging the vectors of rows of blocks of rows of integers into a single vector of rows of integers.
    let new_matrix: Vec<Vec<i32>> = input_stuff_matrix.iter_mut().fold(vec![], |mut acc, line| {
        for i in 0..line[0].len() {  // For each grid row
            let mut new_line: Vec<i32> = vec![];
            for k in 0..line.len() {  // For each grid
                new_line.append(&mut line[k][i]);
            }
            acc.push(new_line);
        }
        acc
    });
    // Display.
    // new_matrix.iter().for_each(|row| {
    //     row.iter().for_each(|x| print!("{:?}", x));
    //     println!();
    // });


    // Calculate distance from end for each point, using the point values as distance scores.
    let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; new_matrix[0].len()]; new_matrix.len()];

    let last_row = new_matrix.len() - 1;
    let last_col = new_matrix[0].len() - 1;

    for row in (0..new_matrix.len()).rev() {
        for col in (0..new_matrix[0].len()).rev() {
            let mut r_val = 0;
            let mut d_val = 0;
            let mut p_val = new_matrix[row][col];

            if row == 0 && col == 0 {  // The starting point is not "entered" so does not contribute to distance.
                p_val = 0;
            }

            if row == last_row && col == last_col {
                dists_from_end[row][col] = p_val;
            }
            else {
                if row < last_row {
                    d_val = p_val + dists_from_end[row + 1][col];
                }
                if col < last_col {
                    r_val = p_val + dists_from_end[row][col + 1];
                }
                if r_val == 0 {
                    dists_from_end[row][col] = d_val;
                }
                else if d_val == 0 {
                    dists_from_end[row][col] = r_val;
                }
                else {
                    if d_val < r_val {
                        dists_from_end[row][col] = d_val;
                    }
                    else {  // If equal, doesn't matter which to pick. If more than get r_val.
                        dists_from_end[row][col] = r_val;
                    }
                }
            }
        }
    }
    // Display.
    println!("SCORE: {}", std::cmp::min(dists_from_end[0][1], dists_from_end[1][0]));
    // println!("SCORE: {}", dists_from_end[0][0]);
    // println!("{:?}", dists_from_end);
    // dists_from_end.iter().for_each(|line| {
    //     line.iter().for_each(|x| print!("{:2} ", x));
    //     println!();
    // });


    // Map it! Follow the blocks according to least distance. Gather these points as a path.
    let mut curr: (i32, i32) = (0, 0);
    let mut path: Vec<(i32, i32)> = vec![];
    while curr != (last_row as i32, last_col as i32) {
        let mut d_val = 0;
        if curr.0 < last_row as i32 {
            d_val = dists_from_end[(curr.0 + 1) as usize][curr.1 as usize];
        }

        let mut r_val = 0;
        if curr.1 < last_col as i32 {
            r_val = dists_from_end[curr.0 as usize][(curr.1 + 1) as usize];
        }

        if r_val == 0 {
            curr = (curr.0 + 1, curr.1);
        }
        else if d_val == 0 {
            curr = (curr.0, curr.1 + 1);
        }
        else {
            if r_val < d_val {
                curr = (curr.0, curr.1 + 1);
            }
            else {
                curr = (curr.0 + 1, curr.1);
            }
            // else if d_val < r_val {
            //     curr = (curr.0 + 1, curr.1);
            // }
            // else {  // d_val == r_val -- Try to 'peek' ahead...
                // println!("EQUAL {} {} {} {} {}", row, col, d_val, r_val, p_val);
                // let big_num = 9999;

                // let mut r_d_val = big_num;
                // let mut r_d_val_c = big_num;
                // let mut r_d_val_r = big_num;
                // if row < (last_row + 1) {
                //     r_d_val_c = dists_from_end[row + 2][col];
                // }
                // if row < last_row && col < last_col {
                //     r_d_val_r = dists_from_end[row + 1][col + 1];
                // }
                // r_d_val = std::cmp::min(r_d_val_c, r_d_val_r);

                // let mut c_d_val = big_num;
                // let mut c_d_val_c = big_num;
                // let mut c_d_val_r = big_num;
                // if col < (last_col + 1) {
                //     c_d_val_c = dists_from_end[row][col + 2];
                // }
                // if col < last_col && row < last_row {
                //     c_d_val_r = dists_from_end[row + 1][col + 1];
                // }
                // c_d_val = std::cmp::min(c_d_val_c, c_d_val_r);

                // if r_d_val < c_d_val {
                //     curr = (curr.0 + 1, curr.1);
                // }
                // else {
                //     curr = (curr.0, curr.1 + 1);
                // }
            // }
        }
        path.push(curr);
    }
    // Display.
    // println!("PATH: {:?}", path);
    // for row in 0..new_matrix.len() {
    //     for col in 0..new_matrix[0].len() {
    //         if path.contains(&(row as i32, col as i32)) {
    //             print!("{:1}*", new_matrix[row][col]);
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!("{:1}*", new_matrix[row][col]);
    //             }
    //             else {
    //                 print!("{:1} ", new_matrix[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    let total_score: i32 = path.iter().fold(0, | acc, x | acc + new_matrix[x.0 as usize][x.1 as usize]);
    println!("SCORE: {}", total_score);
}