pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "v...>>.vv>",
    //     ".vv>>.vv..",
    //     ">>.>v>...v",
    //     ">>v>>.>.v.",
    //     "v>v.vv.v..",
    //     ">.>>..v...",
    //     ".vv..>.>v.",
    //     "v.v..>>v.v",
    //     "....v..v.>",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("25.in.txt");

    let mut input_stuff_vec: Vec<Vec<char>> = input_stuff.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
    // println!("{:?}", input_stuff_vec);
    // input_stuff_vec.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));

    let mut steps = 0;
    loop {
        steps += 1;
        let mut movement = 0;
        let mut new_state = input_stuff_vec.clone();
        for (y, line) in input_stuff_vec.iter().enumerate() {
            for (x, point) in line.iter().enumerate() {
                if point == &'.' {
                    continue;
                }
                if point == &'>' {
                    let mut new_x = x + 1;
                    if new_x > (input_stuff_vec[y].len() - 1) {
                        new_x = 0;
                    }
                    if input_stuff_vec[y][new_x] == '.' {
                        new_state[y][new_x] = '>';
                        new_state[y][x] = '.';
                        movement += 1;
                    }
                }
            }
        }
        input_stuff_vec = new_state;

        new_state = input_stuff_vec.clone();
        for (y, line) in input_stuff_vec.iter().enumerate() {
            for (x, point) in line.iter().enumerate() {
                if point == &'.' {
                    continue;
                }
                if point == &'v' {
                    let mut new_y = y + 1;
                    if new_y > (input_stuff_vec.len() - 1) {
                        new_y = 0;
                    }
                    if input_stuff_vec[new_y][x] == '.' {
                        new_state[new_y][x] = 'v';
                        new_state[y][x] = '.';
                        movement += 1;
                    }
                }
            }
        }
        input_stuff_vec = new_state;

        // Display.
        // println!();
        // input_stuff_vec.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));

       if movement == 0 {
            break;
       }
    }

    // input_stuff_vec.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));
    println!("MOVES {}", steps);
}