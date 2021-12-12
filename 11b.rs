pub mod usrlib;

fn main() {
    let input_stuff = [
    //     "5483143223",
    //     "2745854711",
    //     "5264556173",
    //     "6141336146",
    //     "6357385478",
    //     "4167524645",
    //     "2176841721",
    //     "6882881134",
    //     "4846848554",
    //     "5283751526",
    //     // "11111",
    //     // "19991",
    //     // "19191",
    //     // "19991",
    //     // "11111",
        "4781623888",
        "1784156114",
        "3265645122",
        "4371551414",
        "3377154886",
        "7882314455",
        "6421348681",
        "7175424287",
        "5488242184",
        "2448568261",
    ];

    let mut input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });

    #[allow(dead_code)]
    fn print_vec(grid: &Vec<Vec<i32>>) {
        for line in grid {
            let line_str: String = line.iter().map(|x| x.to_string()).collect();
            println!("{}", line_str);
        }
    }

    fn step(grid: &mut Vec<Vec<i32>>) {
        for i in 0..grid.len() {
            for k in 0..grid[i].len() {
                grid[i][k] += 1;
            }
        }
    }

    fn flash_point(i: usize, k: usize, grid: &mut Vec<Vec<i32>>, points_flashed: &mut Vec<Vec<usize>>) {
        if grid[i][k] > 9  && !points_flashed.contains(&vec![i, k]) {  // If flash
            points_flashed.push(vec![i, k]);

            if i > 0 {  // Above row
                if k > 0 {
                    grid[i-1][k-1] += 1;
                    flash_point(i-1, k-1, grid, points_flashed);
                }
                grid[i-1][k] += 1;
                flash_point(i-1, k, grid, points_flashed);
                if k < (grid[i].len() - 1) {
                    grid[i-1][k+1] += 1;
                    flash_point(i-1, k+1, grid, points_flashed);
                }
            }
            
            // Same row
            if k > 0 {
                grid[i][k-1] += 1;
                flash_point(i, k-1, grid, points_flashed);
            }
            if k < (grid[i].len() - 1) {
                grid[i][k+1] += 1;
                flash_point(i, k+1, grid, points_flashed);
            }

            if i < (grid[i].len() - 1) {  // Below row
                if k > 0 {
                    grid[i+1][k-1] += 1;
                    flash_point(i+1, k-1, grid, points_flashed);
                }
                grid[i+1][k] += 1;
                flash_point(i+1, k, grid, points_flashed);
                if k < (grid[i].len() - 1) {
                    grid[i+1][k+1] += 1;
                    flash_point(i+1, k+1, grid, points_flashed);
                }
            }
        }
    }

    fn find_flashes(grid: &mut Vec<Vec<i32>>) {
        let mut points_flashed: Vec<Vec<usize>> = vec![];

        for i in 0..grid.len() {
            for k in 0..grid[i].len() {
                flash_point(i, k, grid, &mut points_flashed);
            }
        }
    }

    fn reset(grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut num_flashes = 0;
        for line in grid {
            for point in line {
                if *point > 9 {
                    *point = 0;
                    num_flashes += 1;
                }
            }
        }
        return num_flashes;
    }

    // Not used because it's probably faster to use the number of flashes to determine when to stop, instead of checking for all 0's.
    // fn all_flashing(grid: &Vec<Vec<i32>>) -> bool {
    //     for line in grid {
    //         for point in line {
    //             if *point != 0 as i32 {
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }

    // let limit = 100;  // No limit!
    // let mut total_flashes = 0;
    let mut cycles = 0;
    loop {
        cycles += 1;
        // println!("{}:", cycles);

        step(&mut input_stuff_vec);
        find_flashes(&mut input_stuff_vec);
        let total_flashes = reset(&mut input_stuff_vec);

        // print_vec(&input_stuff_vec);
        // println!("FLASHES: {}", total_flashes);

        if total_flashes == 100 {
            break;
        }

        // Probably faster to just count the number of flashes than to go back and check the matrix.
        // if all_flashing(&input_stuff_vec) {
        //     break;
        // }
    }

    // println!("TOTAL: {}", total_flashes);
    println!("TOTAL: {}", cycles);
}