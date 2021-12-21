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
    // Tricky thing about this input is that it has a '#' at the beginning and '.' at the end of the algo, which means the infinite spaces toggles every cycle.
    let input_stuff = usrlib::vec_lines_from_file("20.in.txt");

    let algo: Vec<char> = input_stuff[0].chars().collect();
    // println!("ALGO: {:?}", algo);
    let mut input_stuff_vec: VecDeque<VecDeque<char>> = input_stuff[2..].iter().fold(VecDeque::new(), | mut acc, line | {
        acc.push_back(line.chars().collect::<VecDeque<char>>());
        acc
    });

    // Initial expansion always has '.' around the border. After this, every expansion of the view depends on char at the border.
    let initial_expand_char = '.';
    for idx in 0..input_stuff_vec.len() {
        input_stuff_vec[idx].push_front(initial_expand_char);
        input_stuff_vec[idx].push_back(initial_expand_char);
    }
    input_stuff_vec.push_front(VecDeque::from(vec![initial_expand_char; input_stuff_vec[0].len()]));
    input_stuff_vec.push_back(VecDeque::from(vec![initial_expand_char; input_stuff_vec[0].len()]));
    // println!("INPUT: {:?}", input_stuff_vec);

    // Add padding to grow into. Gets current border char to determine what char is used for expansion.
    fn expand(input_stuff_vec: &mut VecDeque<VecDeque<char>>) {
        let expand_char = input_stuff_vec[0][0];  // Expand with existing infinite marker.
        for idx in 0..input_stuff_vec.len() {
            input_stuff_vec[idx].push_front(expand_char);
            input_stuff_vec[idx].push_back(expand_char);
        }
        input_stuff_vec.push_front(VecDeque::from(vec![expand_char; input_stuff_vec[0].len()]));
        input_stuff_vec.push_back(VecDeque::from(vec![expand_char; input_stuff_vec[0].len()]));
    }
    // println!("INPUT: {:?}", input_stuff_vec);
    // print_grid(&input_stuff_vec);

    // Opposite of expand.
    // fn reduce(input_stuff_vec: &mut VecDeque<VecDeque<char>>) {
    //     for idx in 0..input_stuff_vec.len() {
    //         input_stuff_vec[idx].pop_front();
    //         input_stuff_vec[idx].pop_back();
    //     }
    //     input_stuff_vec.pop_front();
    //     input_stuff_vec.pop_back();
    // }

    // Get the neighbor values as a binary string.
    fn get_neighbors(input_stuff_vec: &VecDeque<VecDeque<char>>, in_row: usize, in_col: usize) -> String {
        let mut result = "".to_string();
        let this_char_num = if input_stuff_vec[in_row][in_col] == '.' {"0"} else {"1"};  // What does infinite space have? Depends on the border char.

        for row_raw in 0..3 {

            // Deal with space beyond current grid borders.
            if row_raw == 0 && in_row == 0 {  // First row doesn't have a row above it -- it's just whatever char is infinite.
                result.push_str(&this_char_num.to_string().repeat(3));
                continue;
            }
            else if row_raw == 2 && in_row == (input_stuff_vec.len() - 1) {  // Below last row also just repeats infinite char.
                result.push_str(&this_char_num.to_string().repeat(3));
                continue;
            }

            let row: i32 = row_raw - 1;
            for col_raw in 0..3 {
                let col: i32 = col_raw - 1;

                // Deal with space beyond current grid borders.
                if col_raw == 0 && in_col == 0 {
                    result.push_str(&this_char_num.to_string());
                    continue;
                }
                else if col_raw == 2 && in_col == (input_stuff_vec[0].len() - 1) {
                    result.push_str(&this_char_num.to_string());
                    continue;
                }

                let neighbor = input_stuff_vec[(in_row as i32 + row) as usize][(in_col as i32 + col) as usize];
                result.push_str(if neighbor == '.' {"0"} else {"1"});
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
        expand(input_stuff_vec);  // Expand once, since each cycle grows out the grid.
        let mut new_stuff_vec = input_stuff_vec.clone();  // Need to have a copy of the original while constructing the "new" replacement.
        for row in 0..input_stuff_vec.len() {
            for col in 0..input_stuff_vec[0].len() {
                let neighbors_binary: String = get_neighbors(&input_stuff_vec, row, col);
                let neighbors_num = isize::from_str_radix(&neighbors_binary, 2).unwrap();  // Get integer from binary string.
                let neighbors_char = algo[neighbors_num as usize];  // Lookup on the algo table.
                // println!("{} {} {}", neighbors_binary, neighbors_num, neighbors_char);
                new_stuff_vec[row][col] = neighbors_char;  // Do the replacement.
            }
        }
        *input_stuff_vec = new_stuff_vec;
    }

    // let num_cycles = 2;
    let num_cycles = 50;

    for _ in 0..num_cycles {
        cycle(&mut input_stuff_vec, &algo);
        // print_grid(&input_stuff_vec);
    }

    // Count the lit pixels!
    let mut num_lit_pixels = 0;
    for row in 0..(input_stuff_vec.len() - 0) {
        for col in 0..(input_stuff_vec[0].len() - 0) {
            if input_stuff_vec[row][col] == '#' {
                num_lit_pixels += 1;
            }
        }
    }

    // print_grid(&input_stuff_vec);
    println!("NUM: {}", num_lit_pixels);
}