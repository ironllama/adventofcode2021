pub mod usrlib;

// DIJKSTRA!
// Source: https://doc.rust-lang.org/stable/std/collections/binary_heap/index.html
// Just updated so that positions are (x, y) instead of just x.
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))  // Ends up just being a lexical tuple compare.
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: (usize, usize),
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn dijkstra(adj_list: &HashMap<(usize, usize), Vec<Edge>>, start: (usize, usize), goal: (usize, usize)) -> Option<(usize, Vec<(usize, usize)>)> {
    // dist[node] = current shortest distance from `start` to `node`
    // let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    let mut cameFrom: HashMap<&(usize, usize), (usize, usize)> = HashMap::new();

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some((cost, reconstruct_path(&cameFrom, &position))); }

        // Important as we may have already found a better way
        if cost > *dist.entry(position).or_insert(usize::MAX) { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        // println!("STUFF: {:?}", adj_list.get(&position).unwrap());
        for edge in adj_list.get(&position).unwrap() {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < *dist.entry(next.position).or_insert(usize::MAX) {
                // println!("PUSHING: {:?}", next);
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.entry(next.position).or_insert(next.cost) = next.cost;

                *cameFrom.entry(&edge.node).or_insert(position) = position;  // Just for display.
            }
        }
    }
    // Goal not reachable
    None
}

// Taken from 15a and A* wiki algo.
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
    let input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter()
        .fold(vec![], |mut acc, line| {
            let line_ints: Vec<i32> = line.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect();
            acc.push(line_ints);
            acc
        });
   
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

    // Prep the data into a HashMap of points and Edge structures to feed dijkstra().
    let mut input_stuff_edges: HashMap<(usize, usize), Vec<Edge>> = HashMap::new();
    for y in 0..new_matrix.len() {
        for x in 0..new_matrix[y].len() {
            let mut new_vec: Vec<Edge> = vec![];
            if y > 0 {
                new_vec.push(Edge{node: (x, y-1), cost: new_matrix[y-1][x] as usize});
            }
            if y < (new_matrix.len()-1) {
                new_vec.push(Edge{node: (x, y+1), cost: new_matrix[y+1][x] as usize});
            }
            if x > 0 {
                new_vec.push(Edge{node: (x-1, y), cost: new_matrix[y][x-1] as usize});
            }
            if x < (new_matrix[y].len()-1) {
                new_vec.push(Edge{node: (x+1, y), cost: new_matrix[y][x+1] as usize});
            }
            input_stuff_edges.insert((x, y), new_vec);
        }
    }
    // print!("EDGES: ");
    // input_stuff_edges.iter().for_each(|x| println!("{:?}", x));

    let path_tup = dijkstra(&input_stuff_edges, (0, 0), (new_matrix[0].len()-1, new_matrix.len()-1)).unwrap();
    // Display. Draw out the path.
    // for row in 0..new_matrix.len() {
    //     for col in 0..new_matrix[0].len() {
    //         if path_tup.1.contains(&(col, row)) {  // NOTE! new_matrix is [row][col], but path_vec is [x][y] (opposite)
    //             print!("{:2}*", new_matrix[row][col]);
    //         }
    //         else {
    //             if row == 0 && col == 0 {
    //                 print!("{:2}*", new_matrix[row][col]);
    //             }
    //             else {
    //                 print!("{:2} ", new_matrix[row][col]);
    //             }
    //         }
    //     }
    //     println!();
    // }

    println!("LEN: {}", path_tup.0);
}





    // // Calculate distance from end for each point, using the point values as distance scores.
    // let mut dists_from_end: Vec<Vec<i32>> = vec![vec![0; new_matrix[0].len()]; new_matrix.len()];

    // let last_row = new_matrix.len() - 1;
    // let last_col = new_matrix[0].len() - 1;

    // for row in (0..new_matrix.len()).rev() {
    //     for col in (0..new_matrix[0].len()).rev() {
    //         let mut r_val = 0;
    //         let mut d_val = 0;
    //         let mut p_val = new_matrix[row][col];

    //         if row == 0 && col == 0 {  // The starting point is not "entered" so does not contribute to distance.
    //             p_val = 0;
    //         }

    //         if row == last_row && col == last_col {
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
    //                 else {  // If equal, doesn't matter which to pick. If more than get r_val.
    //                     dists_from_end[row][col] = r_val;
    //                 }
    //             }
    //         }
    //     }
    // }
    // // Display.
    // println!("SCORE: {}", std::cmp::min(dists_from_end[0][1], dists_from_end[1][0]));
    // // println!("SCORE: {}", dists_from_end[0][0]);
    // // println!("{:?}", dists_from_end);
    // // dists_from_end.iter().for_each(|line| {
    // //     line.iter().for_each(|x| print!("{:2} ", x));
    // //     println!();
    // // });


    // // Map it! Follow the blocks according to least distance. Gather these points as a path.
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
    //         if r_val < d_val {
    //             curr = (curr.0, curr.1 + 1);
    //         }
    //         else {
    //             curr = (curr.0 + 1, curr.1);
    //         }
    //         // else if d_val < r_val {
    //         //     curr = (curr.0 + 1, curr.1);
    //         // }
    //         // else {  // d_val == r_val -- Try to 'peek' ahead...
    //             // println!("EQUAL {} {} {} {} {}", row, col, d_val, r_val, p_val);
    //             // let big_num = 9999;

    //             // let mut r_d_val = big_num;
    //             // let mut r_d_val_c = big_num;
    //             // let mut r_d_val_r = big_num;
    //             // if row < (last_row + 1) {
    //             //     r_d_val_c = dists_from_end[row + 2][col];
    //             // }
    //             // if row < last_row && col < last_col {
    //             //     r_d_val_r = dists_from_end[row + 1][col + 1];
    //             // }
    //             // r_d_val = std::cmp::min(r_d_val_c, r_d_val_r);

    //             // let mut c_d_val = big_num;
    //             // let mut c_d_val_c = big_num;
    //             // let mut c_d_val_r = big_num;
    //             // if col < (last_col + 1) {
    //             //     c_d_val_c = dists_from_end[row][col + 2];
    //             // }
    //             // if col < last_col && row < last_row {
    //             //     c_d_val_r = dists_from_end[row + 1][col + 1];
    //             // }
    //             // c_d_val = std::cmp::min(c_d_val_c, c_d_val_r);

    //             // if r_d_val < c_d_val {
    //             //     curr = (curr.0 + 1, curr.1);
    //             // }
    //             // else {
    //             //     curr = (curr.0, curr.1 + 1);
    //             // }
    //         // }
    //     }
    //     path.push(curr);
    // }
    // // Display.
    // // println!("PATH: {:?}", path);
    // // for row in 0..new_matrix.len() {
    // //     for col in 0..new_matrix[0].len() {
    // //         if path.contains(&(row as i32, col as i32)) {
    // //             print!("{:1}*", new_matrix[row][col]);
    // //         }
    // //         else {
    // //             if row == 0 && col == 0 {
    // //                 print!("{:1}*", new_matrix[row][col]);
    // //             }
    // //             else {
    // //                 print!("{:1} ", new_matrix[row][col]);
    // //             }
    // //         }
    // //     }
    // //     println!();
    // // }

    // let total_score: i32 = path.iter().fold(0, | acc, x | acc + new_matrix[x.0 as usize][x.1 as usize]);
    // println!("SCORE: {}", total_score);