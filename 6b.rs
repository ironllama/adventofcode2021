pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "3,4,3,1,2"
    // ];
    let input_stuff = usrlib::vec_lines_from_file("6.in.txt");

    let total_cycles = 256;

    let input_vec: Vec<&str> = input_stuff[0].split(',').collect();
    let input_vec_ints: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    // println!("START: {} {:?}", input_vec_ints.len(), input_vec_ints);
    println!("DIGITS: {}", input_vec_ints.len());

    fn get_children(offset: i32, days: i32, num: i32) -> i64 {
        // println!("OFFSET: {} DAYS: {} NUM: {}", offset, days, num);
        let mut total_children: i64 = 0;
        let new_num = num + 1;  // Just to detect the first generation.

        let mut cycle_size = 9;  // First born of each generation starts with an 8 and goes to 0.
        let mut new_days = days - cycle_size;
        if num == 0 {  // First generation does its own thing.
            new_days = days - offset - 1;
        }

        while new_days >= 0 {
            total_children += 1;
            // println!("ADDED NUM: {}", num);
            total_children += get_children(offset, new_days, new_num);

            cycle_size = 7;  // After the first born, the rest of a generation go from 6 to 0.
            new_days -= cycle_size;
        }
        // println!("TOTAL_CHILDREN: {}", total_children);
        return total_children;
    }

    // Get the fish amount for only digits in the input and create a lookup table of amounts.
    let mut digit_index: Vec<i64> = vec![0; 10];
    let unique_nums: Vec<i32> = input_vec_ints.iter().fold(vec![], | mut acc, x | { if !acc.contains(x) { acc.push(*x); } return acc; });
    for idx in unique_nums {
        let num_fish = get_children(idx, total_cycles, 0);
        println!("IDX: {} TOTAL_FISH: {}", idx, num_fish);
        digit_index[idx as usize] = num_fish;
    }

    // Using the lookup table created above, add up the necessary amounts.
    let mut total_fish = input_vec_ints.len() as i64;
    for idx in 0..total_fish {
        // println!("IDX: {} TOTAL_FISH: {}", idx, new_fish);
        let new_fish = digit_index[input_vec_ints[idx as usize] as usize];
        total_fish += new_fish;
    }

    println!("TOTAL: {}", total_fish);
}




    // //// BRUTE FORCE. Yea, shows as impractical after around 150+ cycles.
    // // fn next_day(all_fish: &mut Vec<i32>) -> () {
    // //     for idx in 0..all_fish.len() {
    // //         if all_fish[idx] == 0 {
    // //             all_fish[idx] = 6;
    // //             all_fish.push(8);
    // //         }
    // //         else {
    // //             all_fish[idx] -= 1;
    // //         }
    // //         // println!("F: {}", idx);
    // //     }
    // // }

    // // // for cycles in 1..18 + 1 {
    // // // for cycles in 1..32 + 1 {
    // // // for cycles in 1..80 + 1 {
    // // // for cycles in 1..64 + 1 {
    // // // for cycles in 1..256 + 1 {
    // // println!("CYCLE {:02} {:02} -> {:?}", 0, input_vec_ints.len(), input_vec_ints.iter().map(|x| x.to_string() ).collect::<String>());

    // // let mut new_int_vec = input_vec_ints.clone();
    // // for cycles in 1..(total_cycles + 1) {
    // //     next_day(&mut new_int_vec);
    // //     let result_str = new_int_vec.iter().map(|x| x.to_string()).collect::<String>();
    // //     println!("CYCLE {:02} {:02} -> {:?}", cycles, result_str.len(), result_str);
    // //     // println!("CYCLE {} {}", cycles, result_str.len());
    // //     // println!("CYCLE {}", cycles);
    // // }

    // // // // // println!("FINAL {} {:?}", result.len(), result);
    // // println!("FINAL {}", input_vec_ints.len());

    // // fn run_cycle(target_cycles: i32, input_vec_ints: &Vec<i32>) -> () {
    // //     // let target_cycles = 12;
    // //     let cycle_size = 7;
    // //     let mut total_size = input_vec_ints.len() as i32;
    // //     let mut new_adults = 0;
    // //     let mut new_juveniles = 0;
    // //     for num in input_vec_ints {
    // //         let mut new_children = 0;
    // //         let new_num = (num - target_cycles).rem_euclid(cycle_size);
    // //         // let num_children =  (target_cycles - (cycle_size - num)) / cycle_size;
    // //         // let num_children = (num - target_cycles).abs() / cycle_size;
    // //         let num_cycles = target_cycles + (cycle_size - num) - 1;
    // //         let mut _new_adults = num_cycles / cycle_size;
    // //         // if num_cycles.rem_euclid(cycle_size) == 0 { _new_adults -= 1 }

    // //         let mut _new_juveniles = 0;
    // //         if new_num == 6 {
    // //             _new_adults -= 1;
    // //             _new_juveniles += 1;
    // //         }

    // //         let num_new_child_cycles = target_cycles - (num + 1);
    // //         // println!("{}", num_new_child_cycles);
    // //         // if num_new_child_cycles.rem_euclid(9) > 0 {
    // //             _new_juveniles += num_new_child_cycles / 9;
    // //         // }

    // //         new_adults += _new_adults;
    // //         new_juveniles += _new_juveniles;
    // //         println!("{} {} {} {} {}", num, new_num, _new_adults, _new_juveniles, total_size);
    // //     }

    // //     total_size += new_adults + new_juveniles;

    // //     println!("{} {} {} {}", target_cycles, new_adults, new_juveniles, total_size);
    // // }

    // // for i in 9..(18 + 1) {
    // //     run_cycle(i, &input_vec_ints);
    // // }

    // // fn get_children(birth_day: i32, days: i32, num: i32) -> i32 {
    // fn get_children(offset: i32, days: i32, num: i32) -> i64 {
    //     // println!("OFFSET: {} DAYS: {} NUM: {}", offset, days, num);
    //     let mut total_children: i64 = 0;
    //     let new_num = num + 1;

    //     // let mut cycle_size = 9;
    //     // let mut new_days = days - cycle_size;
    //     // if num == 0 {
    //     //     cycle_size = 7;
    //     //     new_days = days - (cycle_size - offset);
    //     // }

    //     // if new_days > 0 {
    //     //     total_children += 1;
    //     //     total_children += get_children(offset, new_days, new_num);
    //     // }
    //     // println!("NEW_DAYS: {} CYCLE_SIZE: {} FIRST_CHILD {}", new_days, cycle_size, total_children);

    //     // let mut new_days = days;
    //     // let mut cycle_size = 9;
    //     // // if num == 0 {
    //     // //     cycle_size = 9;
    //     // // }

    //     // if new_days >= cycle_size {
    //     //     total_children += 1;

    //     //     if num > 0 {
    //     //         new_days -= cycle_size;
    //     //     }
    //     //     else {
    //     //         cycle_size = 7;
    //     //         new_days -= cycle_size - offset;
    //     //     }
    //     // }
    //     // println!("NEW_DAYS: {} CYCLE_SIZE: {} FIRST_CHILD {}", new_days, cycle_size, total_children);

    //     // let num_cycles = days + cycle_size;
    //     // let num_cycles = days - offset - 1;
    //     // let num_cycles = days - offset;
    //     // total_children += num_cycles / cycle_size;  // Integer divison ignores anything after the point.
    //     // total_children += (days - offset) / cycle_size;  // Integer divison ignores anything after the point.
    //     // total_children += new_adults;
    //     // println!("NUM: {}, COL_SIZE: {} OFFSET: {} DAYS: {} TOTAL_CHILDREN: {}", num, num_cycles, offset, days, total_children);

    //     // let mut new_num = (birth_day - days).rem_euclid(cycle_size);
    //     // let num_cycles = days + (cycle_size - birth_day) - 1;
    //     // let mut _new_adults = num_cycles / cycle_size;

    //     // let mut day_gap = days - cycle_size;
    //     // if num == 0 {
    //     //     day_gap = cycle_size - 2;
    //     //     new_num += 2;
    //     // }

    //     // total_children += _new_adults;
    //     // println!("AGE: {} DAYS: {} NEW_NUM: {} NEW_ADULTS: {}", birth_day, days, new_num, _new_adults);
    //     // println!("OFFSET: {} DAYS: {} NEW_ADULTS: {}", offset, days, total_children);

    //     // if day_gap > 0 {

    //     // cycle_size = 7;

    //     // let more_children = new_days / cycle_size;
    //     // println!("MORE: {}", more_children);

    //     // new_days -= cycle_size;

    //     // let new_days = days;
    //     let mut cycle_size = 9;
    //     let mut new_days = days - cycle_size;
    //     if num == 0 {
    //         // cycle_size = 7;
    //         // new_days = (days + (cycle_size - offset)) - cycle_size;
    //         new_days = days - offset - 1;
    //     }

    //     while new_days >= 0 {
    //         total_children += 1;
    //         // println!("ADDED NUM: {}", num);
    //         // let more_children = (new_days as f32 / cycle_size as f32).ceil() as i32;

    //         let mut new_children: i64 = 0;
    //         new_children += get_children(offset, new_days, new_num);
    //         total_children += new_children;

    //         cycle_size = 7;
    //         new_days -= cycle_size;

    //         // let mut new_offset = 2;
    //         // let mut new_days = days;
    //         // let mut new_num = num;
    //         // for this_child in 0..total_children {
    //         //     let mut new_children = 0;
    //         //     // if offset < 0 {
    //         //     //     new_offset = 2;
    //         //     // }
    //         //     let mut next_days = new_days - cycle_size - new_offset;
    //         //     if (new_num == 0) {
    //         //         next_days = new_days - cycle_size - offset;
    //         //     }
    //         //     new_num += 1;
    //         //     // let next_days = days - cycle_size;
    //         //     // if num == 0 {
    //         //         // day_gap -= 1;
    //         //         // new_num += 2;
    //         //         // new_num -= 1;
    //         //     // }

    //         //     // println!("AGE: {} DAYS: {} NEW_NUM: {} NEW_ADULTS: {}", birth_day, days, new_num, _new_adults);
    //         //     // total_children += _new_adults;
    //         //     // println!("TOTAL_CHILDREN: {}", total_children);
    //         //     // total_children += get_children(new_offset, next_days, new_num);
    //         //     new_children += get_children(new_offset, next_days, new_num);
    //         //     println!("NUM: {}, NEW_OFFSET: {} NEXT_DAYS: {} NEW_ADULTS: {}", num, new_offset, next_days, new_children);
    //         //     new_offset = 0;
    //         //     new_days = next_days;
    //         //     total_children += new_children;
    //         // }
    //     }
    //     // else {
    //     //     println!("END");
    //     // }

    //     // println!("TOTAL_CHILDREN: {}", total_children);
    //     // println!("TOTAL_CHILDREN: 0");
    //     return total_children;
    // }

    // // Get the fish amount for every digit.
    // // For 256 times:
    // // IDX: 0 TOTAL_FISH: 6703087163
    // // IDX: 1 TOTAL_FISH: 6206821032
    // // IDX: 2 TOTAL_FISH: 5617089147
    // // IDX: 3 TOTAL_FISH: 5217223241
    // // IDX: 4 TOTAL_FISH: 4726100873
    // // IDX: 5 TOTAL_FISH: 4368232008
    // // IDX: 6 TOTAL_FISH: 3989468461
    // // IDX: 7 TOTAL_FISH: 3649885551
    // // IDX: 8 TOTAL_FISH: 3369186777
    // // IDX: 9 TOTAL_FISH: 3053201611
    // // let mut digit_index: Vec<i64> = vec![];
    // // for idx in 0..10 {
    // //     let num_fish = get_children(idx, total_cycles, 0);
    // //     println!("IDX: {} TOTAL_FISH: {}", idx, num_fish);
    // //     // println!("");
    // //     digit_index.push(num_fish);
    // // }

    // Get the fish amount for only digits in the input.
    // let mut digit_index: Vec<i64> = vec![0; 10];
    // let unique_nums: Vec<i32> = input_vec_ints.iter().fold(vec![], | mut acc, x | { if !acc.contains(x) { acc.push(*x); } return acc; });
    // for idx in unique_nums {
    //     let num_fish = get_children(idx, total_cycles, 0);
    //     println!("IDX: {} TOTAL_FISH: {}", idx, num_fish);
    //     // println!("");
    //     digit_index[idx as usize] = num_fish;
    // }

    // let mut total_fish = input_vec_ints.len() as i64;
    // for idx in 0..total_fish {
    //     // let new_fish = get_children(input_vec_ints[idx as usize], total_cycles, 0);
    //     // println!("IDX: {} TOTAL_FISH: {}", idx, new_fish);
    //     let new_fish = digit_index[input_vec_ints[idx as usize] as usize];
    //     // println!("");
    //     total_fish += new_fish;
    // }
    // // total_fish += get_children(input_vec_ints[0], 18, 0);


    // println!("TOTAL: {}", total_fish);

    // // let mut result = input_vec_ints;
    // // // for cycles in 1..18 + 1 {
    // // // for cycles in 1..32 + 1 {
    // // // for cycles in 1..80 + 1 {
    // // // for cycles in 1..64 + 1 {
    // // for cycles in 1..256 + 1 {
    // //     result = next_day(result);
    // //     let result_str = result.iter().map(|x| x.to_string()).collect::<String>();
    // //     // println!("CYCLE {} {} -> {:?}", cycles, result_str.len(), result_str);
    // //     println!("CYCLE {} {}", cycles, result_str.len());
    // // }

    // // // println!("FINAL {} {:?}", result.len(), result);
    // // println!("FINAL {}", result.len());
