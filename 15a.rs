pub mod usrlib;

fn main() {
    let input_stuff = [
        // "1163751742",
        // "1381373672",
        // "2136511328",
        // "3694931569",
        // "7463417111",
        // "1319128137",
        // "1359912421",
        // "3125421639",
        // "1293138521",
        // "2311944581",

        // "09111",
        // "22341",
        // "43521",
        // "35627",

        // "0243",
        // "9235",
        // "1322",
        // "1992",
        // "1117",

        "02754564",
        "92911116",
        "11614514",
        "52116611",
        "16434341",
    ];
    // let input_stuff = usrlib::vec_lines_from_file("15.in.txt");

    // Get the input data the way I want to use it -- vector of vector of integers.
    let input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });
    // println!("{:?}", input_stuff_vec);

    // Calculate distance from end for each point, using the point values as distance scores.
    let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; input_stuff_vec[0].len()]; input_stuff_vec.len()];

    let last_row = input_stuff_vec.len() - 1;
    let last_col = input_stuff_vec[0].len() - 1;

    for row in (0..input_stuff_vec.len()).rev() {
        for col in (0..input_stuff_vec[0].len()).rev() {
            let mut r_val = 0;
            let mut d_val = 0;
            let mut p_val = input_stuff_vec[row][col];

            if row == 0 && col == 0 {  // The starting point is not "entered" so does not contribute to distance.
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
                    if d_val < r_val {
                        dists_from_end[row][col] = d_val;
                    }
                    else {
                        // if d_val == r_val {
                        //     println!("EQUAL {} {} {} {} {}", row, col, d_val, r_val, p_val);
                        // }
                        dists_from_end[row][col] = r_val;
                    }
                }
            }
        }
    }
    // Display.
    println!("SCORE: {}", std::cmp::min(dists_from_end[0][1], dists_from_end[1][0]));
    // println!("{:?}", dists_from_end);
    // dists_from_end.iter().for_each(|line| {
    //     line.iter().for_each(|x| print!("{:3} ", x));
    //     println!();
    // });

    // Map it! Follow the blocks according to least distance. Gather these points as a path.
    let mut curr: (i32, i32) = (0, 0);
    let mut path: Vec<(i32, i32)> = vec![];
    while curr != (last_row as i32, last_col as i32) {
        // println!("CURR: {:?}", curr);

        let mut u_val = i32::MAX;
        if curr.0 > 0 {
            u_val = dists_from_end[(curr.0 - 1) as usize][curr.1 as usize];
        }

        let mut d_val = i32::MAX;
        if curr.0 < last_row as i32 {
            d_val = dists_from_end[(curr.0 + 1) as usize][curr.1 as usize];
        }

        let mut l_val = i32::MAX;
        if curr.1 > 0 {
            l_val = dists_from_end[curr.0 as usize][(curr.1 - 1) as usize];
        }

        let mut r_val = i32::MAX;
        if curr.1 < last_col as i32 {
            r_val = dists_from_end[curr.0 as usize][(curr.1 + 1) as usize];
        }
        // println!("VALS: {} {} {} {}", u_val, d_val, l_val, r_val);

        let next_step: i32 = *[u_val, d_val, l_val, r_val].iter().min().unwrap();
        // println!("NS: {:?}", next_step);
        curr = match next_step {
            u if u == u_val => (curr.0 - 1, curr.1),
            l if l == l_val => (curr.0, curr.1 - 1),
            d if d == d_val => (curr.0 + 1, curr.1),
            _ => (curr.0, curr.1 + 1),
            // r if r == r_val => (curr.0, curr.1 + 1),
            // _ => (curr.0, curr.1 - 1),
        };

        // if r_val == 0 {
        //     curr = (curr.0 + 1, curr.1);
        // }
        // else if d_val == 0 {
        //     curr = (curr.0, curr.1 + 1);
        // }
        // else {
        //     if r_val > 0 && r_val < d_val {
        //         curr = (curr.0, curr.1 + 1);
        //     }
        //     else {
        //         curr = (curr.0 + 1, curr.1);
        //     }
        // }
        path.push(curr);
    }
    // Display.
    // println!("PATH: {:?}", path);
    // for row in 0..input_stuff_vec.len() {
    //     for col in 0..input_stuff_vec[0].len() {
    //         if path.contains(&(row as i32, col as i32)) {
    //             print!("{:1}*", input_stuff_vec[row][col]);
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!("{:1}*", input_stuff_vec[row][col]);
    //             }
    //             else {
    //                 print!("{:1} ", input_stuff_vec[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }
    for row in 0..dists_from_end.len() {
        for col in 0..dists_from_end[0].len() {
            if path.contains(&(row as i32, col as i32)) {
                print!("{:3}*", dists_from_end[row][col]);
            }
            else {
                if row == 0 && col == 0 {
                    print!("{:3}*", dists_from_end[row][col]);
                }
                else {
                    print!("{:3} ", dists_from_end[row][col]);
                }
            }
        }
        println!();
    }

    let total_score: i32 = path.iter().fold(0, | acc, x | acc + input_stuff_vec[x.0 as usize][x.1 as usize]);
    println!("SCORE: {}", total_score);
}