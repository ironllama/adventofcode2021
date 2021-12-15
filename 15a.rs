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

    let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; input_stuff_vec[0].len()]; input_stuff_vec.len()];

    let last_row = input_stuff_vec.len() - 1;
    let last_col = input_stuff_vec[0].len() - 1;

    for row in (0..input_stuff_vec.len()).rev() {
        for col in (0..input_stuff_vec[0].len()).rev() {
            let mut r_val = 0;
            let mut d_val = 0;
            let mut p_val = input_stuff_vec[row][col];
            if row == 0 && col == 0 {
                p_val = 0;
            }

            if row == last_row && col == last_col {
                // dists_from_end[row][col] = 1;
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
                    if d_val <= r_val {
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
    // println!("{:?}", dists_from_end);
    // dists_from_end.iter().for_each(|line| {
    //     line.iter().for_each(|x| print!("{:2} ", x));
    //     println!();
    // });

    // Map it!
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
            if r_val > 0 && r_val < d_val {
                curr = (curr.0, curr.1 + 1);
            }
            else {
                curr = (curr.0 + 1, curr.1);
            }
        }
        path.push(curr);
    }
    // // println!("PATH: {:?}", path);
    // for row in 0..input_stuff_vec.len() {
    //     for col in 0..input_stuff_vec[0].len() {
    //         if path.contains(&(row as i32, col as i32)) {
    //             print!(" ");
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!(" ");
    //             }
    //             else {
    //                 print!("{}", input_stuff_vec[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    let total_score: i32 = path.iter().fold(0, | acc, x | acc + input_stuff_vec[x.0 as usize][x.1 as usize]);
    println!("SCORE: {}", total_score);
}