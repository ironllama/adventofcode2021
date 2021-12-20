pub mod usrlib;

use std::collections::VecDeque;

fn main() {
    // let input_stuff = [
    //     "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
    //     // "#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#...",
    //     "",
    //     "#..#.",
    //     "#....",
    //     "##..#",
    //     "..#..",
    //     "..###"
    // ];
    let input_stuff = usrlib::vec_lines_from_file("20.in.txt");

    let algo: Vec<char> = input_stuff[0].chars().collect();
    // println!("ALGO: {:?}", algo);
    let mut input_stuff_vec: VecDeque<VecDeque<char>> = input_stuff[2..].iter().fold(VecDeque::new(), | mut acc, line | {
        acc.push_back(line.chars().collect::<VecDeque<char>>());
        acc
    });
    // println!("INPUT: {:?}", input_stuff_vec);

    // Add padding to grow into.
    fn expand(input_stuff_vec: &mut VecDeque<VecDeque<char>>) {
        for idx in 0..input_stuff_vec.len() {
            input_stuff_vec[idx].push_front('.');
            // input_stuff_vec[idx].push_front('.');
            input_stuff_vec[idx].push_back('.');
            // input_stuff_vec[idx].push_#ack('.');
        }
        input_stuff_vec.push_front(VecDeque::from(vec!['.'; input_stuff_vec[0].len()]));
        // input_stuff_vec.push_front(VecDeque::from(vec!['.'; input_stuff_vec[0].len()]));
        input_stuff_vec.push_back(VecDeque::from(vec!['.'; input_stuff_vec[0].len()]));
        // input_stuff_vec.push_back(VecDeque::from(vec!['.'; input_stuff_vec[0].len()]));
    }
    expand(&mut input_stuff_vec);
    // println!("INPUT: {:?}", input_stuff_vec);
    // print_grid(&input_stuff_vec);

    // Get the neighbor values as a binary string.
    fn get_neighbors(input_stuff_vec: &VecDeque<VecDeque<char>>, in_row: usize, in_col: usize) -> String {
        let mut result = "".to_string();
        for row_raw in 0..3 {
            let row: i32 = row_raw - 1;
            for col_raw in 0..3 {
                let col: i32 = col_raw - 1;
                let neighbor = input_stuff_vec[(in_row as i32 + row) as usize][(in_col as i32 + col) as usize];
                if neighbor == '.' {
                    result.push_str("0");
                }
                else {
                    result.push_str("1");
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    // Just for pretty printing the stuff.
    fn print_grid(input_stuff_vec: &VecDeque<VecDeque<char>>) {
        input_stuff_vec.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));
        println!();
    }

    // Run a cycle!
    fn cycle(input_stuff_vec: &mut VecDeque<VecDeque<char>>, algo: &Vec<char>) {
        expand(input_stuff_vec);
        let mut new_stuff_vec = input_stuff_vec.clone();
        for row in 1..(input_stuff_vec.len() - 1) {
            for col in 1..(input_stuff_vec[0].len() - 1) {
                let neighbors_binary: String = get_neighbors(&input_stuff_vec, row, col);
                let neighbors_num = isize::from_str_radix(&neighbors_binary, 2).unwrap();
                let neighbors_char = algo[neighbors_num as usize];
                // println!("{} {} {}", neighbors_binary, neighbors_num, neighbors_char);
                new_stuff_vec[row][col] = neighbors_char;
            }
        }
        *input_stuff_vec = new_stuff_vec;
    }


    // Hacky fake border to minic infinite.
    // However, the edges are processed, so we need at least a 3 char gap to get a good count.
    let num_fake_infinite = 20;
    for _ in 0..num_fake_infinite {
        expand(&mut input_stuff_vec);
    }

    // Run twice!
    cycle(&mut input_stuff_vec, &algo);
    // print_grid(&input_stuff_vec);

    cycle(&mut input_stuff_vec, &algo);
    // print_grid(&input_stuff_vec);

    // Doesn't take into account the border -- assumes the borders are okay.
    // However, if the algo turns 9 spaces of no light into light and vice-versa, the count gets messed up.
    // let num_lit_pixels = input_stuff_vec
    //     .iter()
    //     .fold(0, |acc, line|
    //         acc + line
    //             .iter()
    //             .fold(0, |line_acc, line_char|
    //                 line_acc + (if line_char == &'#' {1} else {0})
    //             )
    //         );

    let mut num_lit_pixels = 0;
    for row in num_fake_infinite..(input_stuff_vec.len() - num_fake_infinite) {
        for col in num_fake_infinite..(input_stuff_vec[0].len() - num_fake_infinite) {
            if input_stuff_vec[row][col] == '#' {
                num_lit_pixels += 1;
            }
        }
    }
    println!("NUM: {}", num_lit_pixels);
}