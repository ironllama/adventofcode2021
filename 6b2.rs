pub mod usrlib;

use std::collections::VecDeque;

fn main() {
    let before = std::time::Instant::now();
    // let input_stuff = vec![
    //     "3,4,3,1,2"
    // ];
    let input_stuff = usrlib::vec_lines_from_file("6.in.txt");

    let mut input_stuff_vec: Vec<u64> = vec![0; 9];  // Vector as large as the initial life cycle of fish.
    input_stuff[0].split(",").for_each(|x| {  // Number of fish due to replicate on a given number of days out.
        let new_num: i8 = x.parse::<i8>().unwrap();
        input_stuff_vec[new_num as usize] += 1;
    });
    // println!("IN: {:?}", input_stuff_vec);

    let mut schedule: VecDeque<u64> = VecDeque::from(input_stuff_vec);  // Ring buffer!

    for _ in 0..256 {
        let new_fish = schedule[0];  // Need this to know how many fish are going back into day 6 as adults.
        schedule.rotate_left(1);  // Rotation works because new child fish is always the same number as adults on day 0.
        *schedule.get_mut(6).unwrap() += new_fish;  // Put adults back on day 6.
    }

    println!("DONE: {}", schedule.iter().sum::<u64>());
    println!("Elapsed time: {:.2?}", before.elapsed());
}