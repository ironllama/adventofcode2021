pub mod usrlib;  // Yay, abstraction.

fn main() {
    let input_stuff = vec![
        "3,4,3,1,2"
    ];
    // let input_stuff = usrlib::vec_lines_from_file("6.in.txt");

    let total_cycles = 18;

    let input_vec: Vec<&str> = input_stuff[0].split(',').collect();
    // let mut input_vec_ints: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let input_vec_ints: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();
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

    // for cycles in 1..18 + 1 {
    // for cycles in 1..32 + 1 {
    // for cycles in 1..80 + 1 {
    // for cycles in 1..64 + 1 {
    // for cycles in 1..256 + 1 {
    let mut new_int_vec = input_vec_ints.clone();
    for cycles in 1..(total_cycles + 1) {
        next_day(&mut new_int_vec);
        let result_str = new_int_vec.iter().map(|x| x.to_string()).collect::<String>();
        println!("CYCLE {} {} -> {:?}", cycles, result_str.len(), result_str);
        // println!("CYCLE {} {}", cycles, result_str.len());
        // println!("CYCLE {}", cycles);
    }

    // // // // println!("FINAL {} {:?}", result.len(), result);
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

    // fn get_children(birth_day: i32, days: i32, num: i32) -> i32 {
    fn get_children(offset: i32, days: i32) -> i32 {
        // println!("OFFSET: {} DAYS: {}", offset, days);
        let mut total_children = 0;

        let cycle_size = 7;
        // if num == 1 {
        //     cycle_size = 9;
        // }

        // let num_cycles = days + cycle_size;
        total_children += (days - offset - 1) / cycle_size;  // Integer divison ignores anything after the point.
        // total_children += new_adults;

        // let mut new_num = (birth_day - days).rem_euclid(cycle_size);
        // let num_cycles = days + (cycle_size - birth_day) - 1;
        // let mut _new_adults = num_cycles / cycle_size;
        
        // let mut day_gap = days - cycle_size;
        // if num == 0 {
        //     day_gap = cycle_size - 2;
        //     new_num += 2;
        // }

        // total_children += _new_adults;
        // println!("AGE: {} DAYS: {} NEW_NUM: {} NEW_ADULTS: {}", birth_day, days, new_num, _new_adults);
        // println!("OFFSET: {} DAYS: {} NEW_ADULTS: {}", offset, days, total_children);

        // if day_gap > 0 {
        if days > 0 && total_children > 0 {
            let mut new_offset = 0;
            if offset < 0 {
                new_offset = 2;
            } 
            let next_days = days - cycle_size - offset;
            // let next_days = days - cycle_size;
            // if num == 0 {
                // day_gap -= 1;
                // new_num += 2;
                // new_num -= 1;
            // }

            // println!("AGE: {} DAYS: {} NEW_NUM: {} NEW_ADULTS: {}", birth_day, days, new_num, _new_adults);
            // total_children += _new_adults;
            // println!("NEW_OFFSET: {} NEW_DAYS: {} NEW_ADULTS: {}", new_offset, next_days, total_children);
            // println!("TOTAL_CHILDREN: {}", total_children);
            total_children += get_children(new_offset, next_days);
        }

        // println!("TOTAL_CHILDREN: {}", total_children);
        // println!("TOTAL_CHILDREN: 0");
        return total_children;
    }

    let mut total_fish = input_vec_ints.len() as i32;
    for idx in 0..total_fish {
        let new_fish = get_children(input_vec_ints[idx as usize] - 7, total_cycles);
        println!("TOTAL_FISH: {}", new_fish);
        total_fish += new_fish;

    }
    // total_fish += get_children(input_vec_ints[0], 18, 0);


    println!("TOTAL: {}", total_fish);

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