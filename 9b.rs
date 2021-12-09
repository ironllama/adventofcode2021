pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "2199943210",
    //     "3987894921",
    //     "9856789892",
    //     "8767896789",
    //     "9899965678",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("9.in.txt");

    let input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });
    // println!("INPUT: {:?}", input_stuff_vec);

    // Weird function that tries to change a primitive value AND return a value. Obviously a train-wreck.
    // fn is_less(curr: i32, x: i32, y: i32, input_stuff_vec: &Vec<Vec<i32>>, mut different: &bool) -> bool {
    //     if x >= 0 && y >=0 {
    //         let new_x = x as usize;
    //         let new_y = y as usize;
    //         if new_y < (input_stuff_vec.len() - 1) && new_x < (input_stuff_vec[new_y].len() - 1) {
    //             let curr = input_stuff_vec[new_y][new_x];
    //             let neighbor = input_stuff_vec[new_y][new_x];
    //             if neighbor != curr {
    //                 different = &true;
    //             }
    //             if neighbor < curr {
    //                 return true;
    //             }
    //         }
    //     }

    //     return false;
    // }

    // Utility function to get rid of a lot of the repetitive code from part 1.
    fn get_neighbors(x: usize, y: usize, input_stuff_vec: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        if y > 0 {
            result.push(input_stuff_vec[y - 1][x]);
        }
        if x > 0 {
            result.push(input_stuff_vec[y][x - 1]);
        }
        if y < (input_stuff_vec.len() - 1) {
            result.push(input_stuff_vec[y + 1][x]);
        }
        if x < (input_stuff_vec[y].len() - 1) {
            result.push(input_stuff_vec[y][x + 1]);
        }
        result
    }

    // Function that does something similar as part 1 -- finds all the coords for the low points.
    fn get_low_points(input_stuff_vec: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        // let mut lowest = vec![];
        let mut lowest_coords = vec![];
        for y in 0..input_stuff_vec.len() as usize {
            let line = &input_stuff_vec[y];
            for x in 0..line.len() as usize {
                let curr = line[x];
                let neighbors: Vec<i32> = get_neighbors(x, y, &input_stuff_vec);
                // println!("NEIGH: {:?}", neighbors);
                let filtered_neighbors: Vec<&i32> = neighbors.iter().filter(|x| *x < &curr && *x != &curr ).collect();
                // println!("FILTERED: {:?}", filtered_neighbors);

                if filtered_neighbors.len() == 0 {
                    // lowest.push(curr);
                    lowest_coords.push(vec![x, y]);
                }
            }
        }
        // println!("CRDS: {:?}", lowest_coords);
        lowest_coords
    }
    let low_points: Vec<Vec<usize>> = get_low_points(&input_stuff_vec);
    // println!("CRDS: {:?}", low_points);

    // Recursive function that follows each possible path to find the valid values in the basin. DFS, basically.
    fn follow_basin(x: usize, y: usize, input_stuff_vec: &Vec<Vec<i32>>, total_basin: &mut Vec<Vec<usize>>) {
        if y > 0 {
            if input_stuff_vec[y-1][x] != 9 && !total_basin.contains(&vec![x,y-1]) {
                total_basin.push(vec![x,y-1]);
                follow_basin(x, y-1, &input_stuff_vec, total_basin);
            }
        }
        if x > 0 {
            if input_stuff_vec[y][x-1] != 9 && !total_basin.contains(&vec![x-1,y]) {
                total_basin.push(vec![x-1,y]);
                follow_basin(x-1, y, &input_stuff_vec, total_basin);
            }
        }
        if y < (input_stuff_vec.len() - 1) {
            if input_stuff_vec[y+1][x] != 9 && !total_basin.contains(&vec![x,y+1]) {
                total_basin.push(vec![x,y+1]);
                follow_basin(x, y+1, &input_stuff_vec, total_basin);
            }
        }
        if x < (input_stuff_vec[y].len() - 1) {
            if input_stuff_vec[y][x+1] != 9 && !total_basin.contains(&vec![x+1,y]) {
                total_basin.push(vec![x+1,y]);
                follow_basin(x+1, y, &input_stuff_vec, total_basin);
            }
        }
    }

    // let mut total_basin: Vec<Vec<usize>> = vec![];
    let mut basin_sizes: Vec<usize> = vec![];
    for point in low_points {
        // println!("POINT: {:?}", point);
        let mut this_basin: Vec<Vec<usize>> = vec![];
        follow_basin(point[0], point[1], &input_stuff_vec, &mut this_basin);
        // println!("BASIN: {:?}", this_basin);
        basin_sizes.push(this_basin.len());
    }

    // let basin_sizes: Vec<usize> = total_basin.iter().map(|x| x.len()).collect();
    // println!("BASINS: {:?}", total_basin);
    basin_sizes.sort();
    basin_sizes.reverse();
    // println!("BASINS SIZES: {:?}", basin_sizes);

    // println!("TOP 3: {:?}", basin_sizes.iter().take(3).map(|x| *x as i32 ).collect::<Vec<i32>>());
    println!("TOP 3: {:?}", basin_sizes.iter().take(3).map(|x| *x as i32 ).product::<i32>());
}