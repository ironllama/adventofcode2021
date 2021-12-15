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
    let mut input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });
    // println!("{:?}", input_stuff_vec);

    let mut input_stuff_matrix: Vec<Vec<Vec<Vec<i32>>>> = vec![];
    for row in 0..5 {
        let mut row_input = vec![];
        for col in row..(row + 5) {
            let mut col_input = vec![];
            input_stuff_vec.iter().for_each(|line| {
                // let mut new_line: Vec<i32> = vec![];
                // for col in 1..5 {
                    let mut next_line: Vec<i32> = line.iter().map(|x| {
                        let mut new_num = x + col;
                        if new_num > 9 {
                            new_num = new_num - 9;
                        }
                        new_num
                    }).collect();
                    // new_line.push(next_line);
                    col_input.push(next_line);
                // }
            });
            row_input.push(col_input);
        }
        input_stuff_matrix.push(row_input);
    }
    // For Display.
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
    let new_matrix: Vec<Vec<i32>> = input_stuff_matrix.iter_mut().fold(vec![], |mut acc, line| {
        // let mut new_rows: Vec<Vec<i32>> = vec![];
        for i in 0..line[0].len() {  // For each grid row
            let mut new_line: Vec<i32> = vec![];
            for k in 0..line.len() {  // For each grid
                new_line.append(&mut line[k][i]);
            }
            // println!();
            acc.push(new_line);
        }
        // acc.push(new_row);
        acc
    });
    // new_matrix.iter().for_each(|row| {
    //     row.iter().for_each(|x| print!("{:?}", x));
    //     println!();
    // });


    let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; new_matrix[0].len()]; new_matrix.len()];

    let last_row = new_matrix.len() - 1;
    let last_col = new_matrix[0].len() - 1;

    for row in (0..new_matrix.len()).rev() {
        for col in (0..new_matrix[0].len()).rev() {
            let mut r_val = 0;
            let mut d_val = 0;
            if row == last_row && col == last_col {
                dists_from_end[row][col] = new_matrix[row][col];
            }
            else {
                if row != last_row {
                    d_val = new_matrix[row][col] + dists_from_end[row + 1][col];
                }
                if col != last_col {
                    r_val = new_matrix[row][col] + dists_from_end[row][col + 1];
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
                    else {
                        dists_from_end[row][col] = r_val;
                    }
                }
            }
        }
    }
    println!("SCORE: {}", std::cmp::min(dists_from_end[0][1], dists_from_end[1][0]));
    // println!("SCORE: {}", dists_from_end[0][0]);
    // println!("{:?}", dists_from_end);
    // dists_from_end.iter().for_each(|line| {
    //     line.iter().for_each(|x| print!("{:2} ", x));
    //     println!();
    // });


    //// Map it!
    // let mut curr: (i32, i32) = (0, 0);
    // let mut path: Vec<(i32, i32)> = vec![];
    // while curr != (last_row as i32, last_col as i32) {
    //     let mut d_val = 0;
    //     if curr.0 < last_row as i32 {
    //         d_val = dists_from_end[(curr.0 + 1) as usize][curr.1 as usize];
    //     }

    //     let mut r_val = 0;
    //     if curr.1 < last_col as i32 {
    //         r_val = dists_from_end[curr.0 as usize][(curr.1 + 1) as usize];
    //     }

    //     if r_val == 0 {
    //         curr = (curr.0 + 1, curr.1);
    //     }
    //     else if d_val == 0 {
    //         curr = (curr.0, curr.1 + 1);
    //     }
    //     else {
    //         if r_val > 0 && r_val < d_val {
    //             curr = (curr.0, curr.1 + 1);
    //         }
    //         else {
    //             curr = (curr.0 + 1, curr.1);
    //         }
    //     }
    //     path.push(curr);
    // }
    // println!("PATH: {:?}", path);
    // for row in 0..new_matrix.len() {
    //     for col in 0..new_matrix[0].len() {
    //         if path.contains(&(row as i32, col as i32)) {
    //             print!(" ");
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!(" ");
    //             }
    //             else {
    //                 print!("{}", new_matrix[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    // let total_score: i32 = path.iter().fold(0, | acc, x | acc + new_matrix[x.0 as usize][x.1 as usize]);
    // println!("SCORE: {}", total_score);
}