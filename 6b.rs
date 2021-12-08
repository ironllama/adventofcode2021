pub mod usrlib;  // Yay, abstraction.

fn main() {
    let input_stuff = vec![
        "3,4,3,1,2"
    ];
    // let input_stuff = usrlib::vec_lines_from_file("6.in.txt");

    let input_vec: Vec<&str> = input_stuff[0].split(',').collect();
    let mut input_vec_ints: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("START: {} {:?}", input_vec_ints.len(), input_vec_ints);

    // BRUTE FORCE. Yea, shows as impractical after around 150+ cycles.
    fn next_day(all_fish: &mut Vec<i32>) -> () {
        for idx in 0..all_fish.len() {
            if all_fish[idx] == 0 {
                all_fish[idx] = 6;
                all_fish.push(8);
            }
            else {
                all_fish[idx] -= 1;
            }
            // println!("F: {}", idx);
        }
    }

    // // for cycles in 1..18 + 1 {
    // // for cycles in 1..32 + 1 {
    // // for cycles in 1..80 + 1 {
    // // for cycles in 1..64 + 1 {
    for cycles in 1..256 + 1 {
        next_day(&mut input_vec_ints);
        // let result_str = input_vec_ints.iter().map(|x| x.to_string()).collect::<String>();
        // println!("CYCLE {} {} -> {:?}", cycles, result_str.len(), result_str);
        // println!("CYCLE {} {}", cycles, result_str.len());
        println!("CYCLE {}", cycles);
    }

    // // // println!("FINAL {} {:?}", result.len(), result);
    // println!("FINAL {}", input_vec_ints.len());

    // fn run_cycle(target_cycles: i32, input_vec_ints: &Vec<i32>) -> () {
    //     // let target_cycles = 12;
    //     let cycle_size = 7;
    //     let mut total_size = input_vec_ints.len() as i32;
    //     let mut new_adults = 0;
    //     let mut new_juveniles = 0;
    //     for num in input_vec_ints {
    //         let mut new_children = 0;
    //         let new_num = (num - target_cycles).rem_euclid(cycle_size);
    //         // let num_children =  (target_cycles - (cycle_size - num)) / cycle_size;
    //         // let num_children = (num - target_cycles).abs() / cycle_size;
    //         let num_cycles = target_cycles + (cycle_size - num) - 1;
    //         let mut _new_adults = num_cycles / cycle_size;
    //         // if num_cycles.rem_euclid(cycle_size) == 0 { _new_adults -= 1 }

    //         let mut _new_juveniles = 0;
    //         if new_num == 6 {
    //             _new_adults -= 1;
    //             _new_juveniles += 1;
    //         }

    //         let num_new_child_cycles = target_cycles - (num + 1);
    //         // println!("{}", num_new_child_cycles);
    //         // if num_new_child_cycles.rem_euclid(9) > 0 {
    //             _new_juveniles += num_new_child_cycles / 9;
    //         // }

    //         new_adults += _new_adults;
    //         new_juveniles += _new_juveniles;
    //         println!("{} {} {} {} {}", num, new_num, _new_adults, _new_juveniles, total_size);
    //     }

    //     total_size += new_adults + new_juveniles;

    //     println!("{} {} {} {}", target_cycles, new_adults, new_juveniles, total_size);
    // }

    // for i in 9..(18 + 1) {
    //     run_cycle(i, &input_vec_ints);
    // }

    // fn get_children(fish: Vec<i32>, days: i32) {
    //     let mut new_fish =
    //     for idx in 0..fish.len() {

    //     }
    // }

    // let mut result = input_vec_ints;
    // // for cycles in 1..18 + 1 {
    // // for cycles in 1..32 + 1 {
    // // for cycles in 1..80 + 1 {
    // // for cycles in 1..64 + 1 {
    // for cycles in 1..256 + 1 {
    //     result = next_day(result);
    //     let result_str = result.iter().map(|x| x.to_string()).collect::<String>();
    //     // println!("CYCLE {} {} -> {:?}", cycles, result_str.len(), result_str);
    //     println!("CYCLE {} {}", cycles, result_str.len());
    // }

    // // println!("FINAL {} {:?}", result.len(), result);
    // println!("FINAL {}", result.len());
}