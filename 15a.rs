pub mod usrlib;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

use std::collections::HashMap;


// Added for the A*. Basic Manhattan distance.
fn heuristic(curr: &(usize, usize), goal: &(usize, usize)) -> usize {
    return goal.0 + goal.1 - curr.0 - curr.1;  // Since the goal is bottom right, should always be greater than curr.
}

// Translated from Wikipedia pseudocode for A*! Tried to keep same number of lines and format to make it easier to reference original.
// SOURCE: https://en.wikipedia.org/wiki/A*_search_algorithm
#[allow(non_snake_case)]
fn reconstruct_path(cameFrom: &HashMap<&(usize, usize), (usize, usize)>, current: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut new_current = current.to_owned();
    let mut total_path: Vec<(usize, usize)> = vec![new_current];
    while cameFrom.contains_key(&new_current) {
        new_current = *cameFrom.get(&new_current).unwrap();
        total_path.push(new_current);
    }
    return total_path;
}
// A* finds a path from start to goal.
// h is the heuristic function. h(n) estimates the cost to reach goal from node n.
#[allow(non_snake_case)]
fn astar(adj_list: &HashMap<(usize, usize), (usize, Vec<((usize, usize), usize)>)>, start: (usize, usize), goal: (usize, usize)) -> Vec<(usize, usize)> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut openSet = BinaryHeap::new();
    openSet.push((Reverse(0), &start));

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut cameFrom: HashMap<&(usize, usize), (usize, usize)> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut gScore: HashMap<(usize, usize), usize> = HashMap::new();
    gScore.insert(start, 0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how short a path from start to finish can be if it goes through n.
    let mut fScore: HashMap<&(usize, usize), usize> = HashMap::new();  // map with default value of Infinity
    fScore.insert(&start, heuristic(&start, &goal));

    while !openSet.is_empty() {
        // This operation can occur in O(1) time if openSet is a min-heap or a priority queue
        let (Reverse(_curr_fScore), current) = openSet.pop().unwrap();  // the node in openSet having the lowest fScore[] value -- fScore included in push()
        if current == &goal {
            return reconstruct_path(&cameFrom, current);
        }

        let (_, neighbors) = adj_list.get(&current).unwrap();
        // openSet.Remove(current)  // Already done with the pop()
        for (neighbor, n_orig_cost) in neighbors {
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            let mut tentative_gScore = match gScore.get(current) {
                    Some(x) => *x,
                    None => usize::MAX
                };
            tentative_gScore += n_orig_cost;
            let gScore_neighbor = gScore.entry(*neighbor).or_insert(usize::MAX);
            if tentative_gScore < *gScore_neighbor {
                // This path to neighbor is better than any previous one. Record it!
                *cameFrom.entry(neighbor).or_insert(*current) = *current;  // Just for display.
                *gScore_neighbor = tentative_gScore;
                let new_fScore = tentative_gScore + heuristic(neighbor, &goal);
                *fScore.entry(neighbor).or_insert(new_fScore) = new_fScore;
                // if !openSet.contains_key(neighbor) {
                    openSet.push((Reverse(new_fScore), neighbor));
                // }
            }
        }
    }

    // Open set is empty but goal was never reached
    //return failure
    panic!("Oops");
}


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

    // Prep the data into a HashMap of points and tuple structures to feed astar(). Basically, pre-assign all the neighbors (edges).
    let mut input_stuff_edges: HashMap<(usize, usize), (usize, Vec<((usize, usize), usize)>)> = HashMap::new();
    for y in 0..input_stuff_vec.len() {
        for x in 0..input_stuff_vec[y].len() {
            let mut new_vec: Vec<((usize, usize), usize)> = vec![];
            if y > 0 {
                new_vec.push(((x, y-1), input_stuff_vec[y-1][x] as usize));
            }
            if y < (input_stuff_vec.len()-1) {
                new_vec.push(((x, y+1), input_stuff_vec[y+1][x] as usize));
            }
            if x > 0 {
                new_vec.push(((x-1, y), input_stuff_vec[y][x-1] as usize));
            }
            if x < (input_stuff_vec[y].len()-1) {
                new_vec.push(((x+1, y), input_stuff_vec[y][x+1] as usize));
            }
            input_stuff_edges.insert((x, y), (input_stuff_vec[y][x] as usize, new_vec));  // Each point stored as tuple: (val, edges[])
        }
    }
    // print!("EDGES: ");
    // input_stuff_edges.iter().for_each(|x| println!("{:?}", x));

    let path_vec = astar(&input_stuff_edges, (0, 0), (input_stuff_vec[0].len()-1, input_stuff_vec.len()-1));
    // Display. Draw out the path.
    // for row in 0..input_stuff_vec.len() {
    //     for col in 0..input_stuff_vec[0].len() {
    //         if path_vec.contains(&(col, row)) {  // NOTE! input_stuff_vec is [row][col], but path_vec is [x][y] (opposite)
    //             print!("{:2}*", input_stuff_vec[row][col]);
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!("{:2}*", input_stuff_vec[row][col]);
    //             }
    //             else {
    //                 print!("{:2} ", input_stuff_vec[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    let path_val = path_vec.iter().fold(0, |acc, point| if *point == (0, 0) {acc} else {acc + input_stuff_vec[point.1][point.0]});  // Remember to not count origin in total!
    println!("LEN: {:?}", path_val);
}









    // // Calculate distance from end for each point, using the point values as distance scores.
    // let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; input_stuff_vec[0].len()]; input_stuff_vec.len()];

    // let last_row = input_stuff_vec.len() - 1;
    // let last_col = input_stuff_vec[0].len() - 1;

    // // Create a "heat-map" or distance map from the end of the puzzle, using the point values as weights.
    // for row in (0..input_stuff_vec.len()).rev() {
    //     for col in (0..input_stuff_vec[0].len()).rev() {
    //         let mut r_val = 0;
    //         let mut d_val = 0;
    //         let mut p_val = input_stuff_vec[row][col];

    //         if row == 0 && col == 0 {  // The starting point is not "entered" so does not contribute to distance.
    //             p_val = 0;
    //         }

    //         if row == last_row && col == last_col {
    //             // dists_from_end[row][col] = 1;
    //             dists_from_end[row][col] = p_val;
    //         }
    //         else {
    //             if row < last_row {
    //                 d_val = p_val + dists_from_end[row + 1][col];
    //             }
    //             if col < last_col {
    //                 r_val = p_val + dists_from_end[row][col + 1];
    //             }
    //             if r_val == 0 {
    //                 dists_from_end[row][col] = d_val;
    //             }
    //             else if d_val == 0 {
    //                 dists_from_end[row][col] = r_val;
    //             }
    //             else {
    //                 if d_val < r_val {
    //                     dists_from_end[row][col] = d_val;
    //                 }
    //                 else {
    //                     // if d_val == r_val {
    //                     //     println!("EQUAL {} {} {} {} {}", row, col, d_val, r_val, p_val);
    //                     // }
    //                     dists_from_end[row][col] = r_val;
    //                 }
    //             }
    //         }
    //     }
    // }

    // // Display.
    // println!("SCORE: {}", std::cmp::min(dists_from_end[0][1], dists_from_end[1][0]));
    // // println!("{:?}", dists_from_end);
    // // dists_from_end.iter().for_each(|line| {
    // //     line.iter().for_each(|x| print!("{:3} ", x));
    // //     println!();
    // // });

    // // Map it! Follow the blocks according to least distance. Gather these points as a path.
    // let mut curr: (i32, i32) = (0, 0);
    // let mut path: Vec<(i32, i32)> = vec![];
    // while curr != (last_row as i32, last_col as i32) {
    //     // println!("CURR: {:?}", curr);

    //     let mut u_val = i32::MAX;
    //     if curr.0 > 0 {
    //         u_val = dists_from_end[(curr.0 - 1) as usize][curr.1 as usize];
    //     }

    //     let mut d_val = i32::MAX;
    //     if curr.0 < last_row as i32 {
    //         d_val = dists_from_end[(curr.0 + 1) as usize][curr.1 as usize];
    //     }

    //     let mut l_val = i32::MAX;
    //     if curr.1 > 0 {
    //         l_val = dists_from_end[curr.0 as usize][(curr.1 - 1) as usize];
    //     }

    //     let mut r_val = i32::MAX;
    //     if curr.1 < last_col as i32 {
    //         r_val = dists_from_end[curr.0 as usize][(curr.1 + 1) as usize];
    //     }
    //     // println!("VALS: {} {} {} {}", u_val, d_val, l_val, r_val);

    //     let next_step: i32 = *[u_val, d_val, l_val, r_val].iter().min().unwrap();
    //     // println!("NS: {:?}", next_step);
    //     curr = match next_step {
    //         u if u == u_val => (curr.0 - 1, curr.1),
    //         l if l == l_val => (curr.0, curr.1 - 1),
    //         d if d == d_val => (curr.0 + 1, curr.1),
    //         _ => (curr.0, curr.1 + 1),
    //         // r if r == r_val => (curr.0, curr.1 + 1),
    //         // _ => (curr.0, curr.1 - 1),
    //     };

    //     // if r_val == 0 {
    //     //     curr = (curr.0 + 1, curr.1);
    //     // }
    //     // else if d_val == 0 {
    //     //     curr = (curr.0, curr.1 + 1);
    //     // }
    //     // else {
    //     //     if r_val > 0 && r_val < d_val {
    //     //         curr = (curr.0, curr.1 + 1);
    //     //     }
    //     //     else {
    //     //         curr = (curr.0 + 1, curr.1);
    //     //     }
    //     // }
    //     path.push(curr);
    // }

    // // Display.
    // // println!("PATH: {:?}", path);
    // // for row in 0..input_stuff_vec.len() {
    // //     for col in 0..input_stuff_vec[0].len() {
    // //         if path.contains(&(row as i32, col as i32)) {
    // //             print!("{:1}*", input_stuff_vec[row][col]);
    // //         }
    // //         else {
    // //             if row == 0 && col == 0 {
    // //                 print!("{:1}*", input_stuff_vec[row][col]);
    // //             }
    // //             else {
    // //                 print!("{:1} ", input_stuff_vec[row][col]);
    // //             }
    // //         }
    // //     }
    // //     println!();
    // // }
    // for row in 0..dists_from_end.len() {
    //     for col in 0..dists_from_end[0].len() {
    //         if path.contains(&(row as i32, col as i32)) {
    //             print!("{:3}*", dists_from_end[row][col]);
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!("{:3}*", dists_from_end[row][col]);
    //             }
    //             else {
    //                 print!("{:3} ", dists_from_end[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    // let total_score: i32 = path.iter().fold(0, | acc, x | acc + input_stuff_vec[x.0 as usize][x.1 as usize]);
    // println!("SCORE: {}", total_score);