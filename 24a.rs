pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "inp z",
    //     "inp x",
    //     "mul z 3",
    //     "eql z x",
    //     // "inp w",
    //     // "add z w",
    //     // "mod z 2",
    //     // "div w 2",
    //     // "add y w",
    //     // "mod y 2",
    //     // "div w 2",
    //     // "add x w",
    //     // "mod x 2",
    //     // "div w 2",
    //     // "mod w 2",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("24.in.txt");

    let num_pos = 14;
    let lines_per_section = 18;
    let ops_lines = vec![4, 5, 15];  // Position of the varying numbers of each instruction section.

    // Vector containing only the varying numbers per instruction section.
    let mut ops: Vec<Vec<i64>> = vec![vec![]; ops_lines.len()];
    for this_pos in 0..num_pos {
        for (idx, op_num) in ops_lines.iter().enumerate() {  // Indexes of the varying numbers of each section.
            let offset = lines_per_section * this_pos;
            ops[idx].push(input_stuff[offset + op_num].split_whitespace().last().unwrap().parse().unwrap());
        }
    }
    // println!("OPS: {:?}", ops);

    // Based on patterns observed on paper. 7 sections of *26 and 7 sections of /26 -- probably some sort of encoding?
    // Each *26 has to be unwound to /26 to start and end at the same number: 0.
    // The modifiers used for push/pull to and from the stack were most likely two of the vectors above.
    let mut stack: Vec<usize> = vec![0 as usize];
    let mut stack_iter: usize = 0;
    let mut final_num: Vec<i64> = vec![0; 14];
    while !stack.is_empty() {
        stack_iter += 1;
        if ops[0][stack_iter] == 26 {
            let idx_one = stack.pop().unwrap() as usize;
            let diff = ops[2][idx_one] + ops[1][stack_iter];  // Diff the modifiers to determine < or > than 0.
            if diff < 0 {
                final_num[idx_one] = 9;
                final_num[stack_iter] = 9 + diff;
            }
            else {
                final_num[idx_one] = 9 - diff;
                final_num[stack_iter] = 9;
            }
        }
        else {
            stack.push(stack_iter);
        }
    }
    print!("FINAL NUM: ");
    final_num.iter().for_each(|x| print!("{}", x));
    println!();
}


    // let mut input_stuff_vec: Vec<Vec<Vec<&str>>> = vec![];
    // let mut temp_vec: Vec<Vec<&str>> = vec![];
    // for idx in 0..input_stuff.len() {
    //     let oper: String = input_stuff[idx].chars().take(3).collect::<String>();
    //     if oper == "inp" {
    //         if !temp_vec.is_empty() {
    //             input_stuff_vec.push(temp_vec);
    //             temp_vec = vec![];
    //         }
    //     }
    //     let new_vec: Vec<&str> = input_stuff[idx].split(' ').collect::<Vec<&str>>();
    //     temp_vec.push(new_vec);
    // }
    // input_stuff_vec.push(temp_vec);  // Push in the last one!

    // // Display.
    // // for (idxs, inst_set) in input_stuff_vec.iter().enumerate() {
    // //     println!("SET {}", idxs);
    // //     inst_set.iter().for_each(|line| println!("\t{:?}", line));
    // // }


    // // Cycle through one set of instructions with the given input.
    // fn run_one(input_stuff_vec: &Vec<Vec<&str>>, wxyz: &mut Vec<i64>, input_num: i64) {
    //     let all_pos: Vec<&str> = vec!["w", "x", "y", "z"];

    //     // let return_num: i32 = input_stuff_vec.iter().fold(0, |mut acc, line| {
    //     input_stuff_vec.iter().for_each(|line| {
    //         let mut first_var_pos: usize = 0;
    //         let first_var_val: i64;
    //         if all_pos.contains(&line[1]) {  // The first var is a char / register position.
    //             first_var_pos = all_pos.iter().position(|&pos| pos == line[1]).unwrap();
    //             first_var_val = wxyz[first_var_pos];
    //         }
    //         else {  // The first var is a number.
    //             first_var_val = line[1].parse::<i64>().unwrap();
    //         }

    //         if line[0] != "inp" {
    //             let second_var_pos: usize;
    //             let second_var_val: i64;
    //             if all_pos.contains(&line[2]) {  // the second var is a char / register position.
    //                 second_var_pos = all_pos.iter().position(|&pos| pos == line[2]).unwrap();
    //                 second_var_val = wxyz[second_var_pos];
    //             }
    //             else {  // The second var is a number.
    //                 second_var_val = line[2].parse::<i64>().unwrap();
    //             }

    //             let new_num: i64 = match line[0] {
    //                 "add" => first_var_val + second_var_val,
    //                 "mul" => first_var_val * second_var_val,
    //                 "div" => first_var_val / second_var_val,
    //                 "mod" => first_var_val % second_var_val,  // Rust has a remainder operator, not "real" modulo. Differs with negative operands. -1 % 3 = -1 (not 2)
    //                 "eql" => if first_var_val == second_var_val {1} else {0},
    //                 _ => panic!("Dude: {} first:{} second:{}", line[0], first_var_val, second_var_val),
    //             };
    //             wxyz[first_var_pos] = new_num;
    //         }
    //         else {
    //             wxyz[first_var_pos] = input_num;
    //         }
    //         // "inp" => assign_registers(, input_num, input_stuff_vec),
    //     });
    // }

    // #[allow(dead_code)]
    // fn test_num(input_stuff_vec: &Vec<Vec<Vec<&str>>>, in_test: String) {
    //     let in_test_vec: Vec<i64> = in_test.chars().map(|x| x.to_string().parse::<i64>().unwrap()).collect();
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //     for (idx, digit) in in_test_vec.iter().enumerate() {
    //         run_one(&input_stuff_vec[idx], &mut wxyz, *digit);
    //         println!("{} Z:{}", idx, wxyz[3]);
    //     }
    //     println!("FINAL: {:?}", wxyz);
    //     if wxyz[3] == 0 {
    //         println!("WINNER: {} {:?}", in_test, wxyz);
    //     }
    // }
    // // test_num(&input_stuff_vec, "31394569391971".to_string());
    // // test_num(&input_stuff_vec, "99999689999296".to_string());
    // // test_num(&input_stuff_vec, "94394569391971".to_string());  // TOO LOW
    // // test_num(&input_stuff_vec, "94394899391971".to_string());  // TOO LOW
    // // test_num(&input_stuff_vec, "97599595979395".to_string());
    // // test_num(&input_stuff_vec, "99394899891971".to_string());  // HIGHEST
    // // test_num(&input_stuff_vec, "92171126131911".to_string());  // LOWEST
    // // test_num(&input_stuff_vec, "99394899991999".to_string());
    // // test_num(&input_stuff_vec, "11191118911999".to_string());

    // #[allow(dead_code)]
    // // Instructions translated.
    // fn run_two(ops: &Vec<Vec<i64>>, digit_pos: usize, w: i64, z: i64) -> i64 {
    //     if ((z % 26) + ops[1][digit_pos]) != w {                // x = 1 or if ((z % 26) + ops[1][digit_pos]) != w
    //         return 26 * (z / 1) + (w + ops[2][digit_pos]);      // ((25 * x) + 1) * (z / ops[0][digit_pos]) + (w + ops[2][digit_pos] * x);
    //     } else {                                                // x = 0 or if ((z % 26) + ops[1][digit_pos]) == w
    //         return z / 26;                                      // z / ops[0][digit_pos];
    //     }
    // }

    // // fn run_two_backwards(ops: &Vec<Vec<i64>>, digit_pos: usize, w: i64, target_z: i64) -> i64 {
    // //     if ((z % 26) + ops[1][digit_pos]) != w {  // x = 1
    // //         return 26 * (z / ops[0][digit_pos]) + (w + ops[2][digit_pos]);
    // //     } else {  // x = 0
    // //         return z / ops[0][digit_pos];
    // //     }
    // // }


    // #[allow(dead_code)]
    // fn test_num_two(ops: &Vec<Vec<i64>>, in_test: String) {
    //     let in_test_vec: Vec<i64> = in_test.chars().map(|x| x.to_string().parse::<i64>().unwrap()).collect();
    //     let mut z: i64 = 0;
    //     for (idx, digit) in in_test_vec.iter().enumerate() {
    //         z = run_two(&ops, idx, *digit, z);
    //         // println!("{} Z:{}", idx, z);
    //     }
    //     // println!("FINAL: {:?}", z);
    //     if z == 0 {
    //         println!("WINNER: {} {:?}", in_test, z);
    //     }
    // }
    // // test_num_two(&ops, "94394899391971".to_string());  // TOO LOW
    // // test_num_two(&ops, "97599595976995".to_string());  // TOO LOW
    // // test_num_two(&ops, "97599595979395".to_string());  // TOO LOW
    // // test_num_two(&ops, "99394899891971".to_string());  // HIGHEST
    // // test_num_two(&ops, "92171126131911".to_string());  // LOWEST 


    // // fn cycle(ops: &Vec<Vec<i64>>, digit_pos: usize, z: i64, id_buff: &String) {
    // fn cycle(ops: &Vec<Vec<i64>>, digit_pos: usize, z: i64, id_buff: &mut Vec<char>) {
    //     if digit_pos == 14 {
    //         if z == 0 {  // Finished and found!
    //             println!("FOUND: {}", id_buff.iter().collect::<String>());
    //         }
    //         return;  // Finshed!
    //     }

    //     // let num_26s: u32 = ops[0][digit_pos..].iter().filter(|&x| *x == 26).count() as u32;
    //     // let limit = 26_i64.pow(num_26s);
    //     let limit = 26_i64.pow(4);
    //     // println!("{}: LIMIT {}", digit_pos, limit);
    //     if z > limit {
    //         println!("TOO FAR!");
    //         return;
    //     }

    //     for num in (1..=9).rev() {
    //         // let new_id = id_buff.clone() + &num.to_string();
    //         id_buff[digit_pos] = num.to_string().chars().next().unwrap();
    //         println!("TEST: {}", id_buff.iter().collect::<String>());
    //         let new_z = run_two(&ops, digit_pos, num, z);
    //         // cycle(&ops, digit_pos + 1, new_z, &new_id);
    //         cycle(&ops, digit_pos + 1, new_z, id_buff);
    //     }
    // }
    // // ops.push(vec![])
    // // cycle(&ops, 0, 0, &"".to_string());

    // // let mut id_buff: Vec<char> = vec!['0'; 15];
    // // cycle(&ops, 0, 0, &mut id_buff);


    // // for i in (1..=9).rev() {
    // //     for j in (1..=9).rev() {
    // //         // let new_nums: String = i.to_string() + &j.to_string() + &"394569391971".to_string();
    // //         // let new_nums: String = i.to_string() + &j.to_string() + &"394879391971".to_string();
    // //         let new_nums: String = i.to_string() + &j.to_string() + &"394899391971".to_string();
    // //         test_num(&input_stuff_vec, new_nums);
    // //     }
    // // }

    // // Testing with the binary sample.
    // // let mut wxyz: Vec<i32> = vec![];
    // // for num in 0..15 {
    // //     wxyz = vec![0, 0, 0, 0];
    // //     run_one(&input_stuff_vec, &mut wxyz, num);
    // //     println!("{:?}", wxyz);
    // // }
    // // println!("{:?}", wxyz);

    // // For number series like 1231, 1232, 1233, instead of running the algo for the first three digits again for each possible complete number,
    // // we store the result of the 1, then 2, then 3, and pass it on to all the possible values of the fourth digit.
    // #[allow(dead_code)]
    // fn next_digit(input_stuff_vec: &Vec<Vec<Vec<&str>>>, wxyz: &Vec<i64>, input_num: i64, spot: usize) -> String {
    //     let limit: usize = 13;
    //     // let limit: usize = 2;

    //     let mut final_num: String = "".to_string();
    //     for digit in (1..=9).rev() {
    //         let mut this_wxyz = wxyz.clone();  // Going to pass along the position value with this.
    //         let new_num = (input_num * 10) + digit;
    //         let zero_padding = 10i64.pow((limit - spot) as u32);

    //         // println!("T:{} POS:{}", digit, new_num * zero_padding);

    //         run_one(&input_stuff_vec[spot], &mut this_wxyz, digit);  // Run to get the total so far for the position.
    //         println!("{} {} {:?}", new_num * zero_padding, digit, this_wxyz);

    //         if spot == limit && this_wxyz[3] == 0 {  // Check only 14 digit numbers and if it's valid.
    //             println!("MAX VALID!");
    //             final_num = (new_num * zero_padding).to_string();
    //             break;
    //         }
    //         if spot < limit {
    //             let ret_str = next_digit(&input_stuff_vec, &this_wxyz, new_num, spot + 1);
    //             if ret_str != "" {
    //                 final_num = ret_str;
    //                 break;
    //             }
    //         }
    //     }
    //     return final_num;
    // }

    // #[allow(dead_code)]
    // fn prev_digit(input_stuff_vec: &Vec<Vec<Vec<&str>>>, wxyz: &Vec<i64>, input_num: i64, spot: usize) -> String {
    //     // let limit: usize = 13;
    //     let limit: usize = 0;

    //     let mut final_num: String = "".to_string();
    //     for digit in (1..=9).rev() {
    //         let mut this_wxyz = wxyz.clone();  // Going to pass along the position value with this.
    //         let new_num = (input_num * 10) + digit;
    //         let zero_padding = 10i64.pow((spot - limit) as u32);

    //         // println!("T:{} POS:{}", digit, new_num * zero_padding);

    //         run_one(&input_stuff_vec[spot], &mut this_wxyz, digit);  // Run to get the total so far for the position.
    //         println!("{} {} {:?}", new_num * zero_padding, digit, this_wxyz);

    //         if spot == limit && this_wxyz[3] == 0 {  // Check only 14 digit numbers and if it's valid.
    //             println!("MAX VALID!");
    //             final_num = (new_num * zero_padding).to_string();
    //             break;
    //         }
    //         if spot > limit {
    //             let ret_str = prev_digit(&input_stuff_vec, &this_wxyz, new_num, spot - 1);
    //             if ret_str != "" {
    //                 final_num = ret_str;
    //                 break;
    //             }
    //         }
    //     }
    //     return final_num;
    // }

    // #[allow(dead_code)]
    // fn maybe_monad(input_stuff_vec: &Vec<Vec<Vec<&str>>>, wxyz: &Vec<i64>, input_num: i64, target: i64, spot: usize) -> String {
    //     let limit: usize = 0;
    //     // let limit: usize = 9;

    //     let mut final_num: String = "".to_string();
    //     for digit in (1..=9).rev() {
    //         let mut this_wxyz = wxyz.clone();
    //         let new_num = (input_num * 10) + digit;
    //         // let zero_padding = 10i64.pow((limit - spot) as u32);
    //         let zero_padding = 10i64.pow((spot - limit) as u32);

    //         let mod_target = digit - input_stuff_vec[spot][5][2].parse::<i64>().unwrap();
    //         let mut new_target = (26 * (target + 1)) - (26 - mod_target);
    //         println!("SPOT:{}, TARGET:{} NEW_MOD:{} NEW_TARGET:{}", spot, target, mod_target, new_target);
    //         // if target == 0 {
    //         //     new_target = mod_target;
    //         // }
    //         let mut test_wxyz: Vec<i64> = vec![0, 0, 0, new_target];
    //         run_one(&input_stuff_vec[spot], &mut test_wxyz, digit);  // Run to get the total so far for the position.
    //         println!("{} {} {:?}", new_num * zero_padding, digit, test_wxyz);
    //         if test_wxyz[3] == target {
    //             if spot > limit {
    //                 let ret_str = maybe_monad(&input_stuff_vec, &this_wxyz, new_num, new_target, spot - 1);
    //                 if ret_str != "" {
    //                     final_num += &ret_str;
    //                 }
    //                 else {
    //                     new_target = ((target as f32 / 26f32).ceil() as i64) - digit;
    //                     println!("\tSPOT:{} TARGET:{} ALT_NEW_TARGET:{}", spot, target, new_target);
    //                     // test_wxyz = vec![0, 0, 0, new_target];
    //                     // run_one(&input_stuff_vec[spot], &mut test_wxyz, digit);  // Run to get the total so far for the position.
    //                     // println!("\t{} {} {:?}", new_num * zero_padding, digit, test_wxyz);
    //                     // if test_wxyz[3] == target {
    //                         if spot > limit {
    //                             let ret_str = maybe_monad(&input_stuff_vec, &this_wxyz, new_num, new_target, spot - 1);
    //                             if ret_str != "" {
    //                                 final_num += &ret_str;
    //                             }
    //                         }
    //                     // }
    //                 }
    //             }
    //             final_num += &digit.to_string();
    //             break;
    //         }
    //         // else {
    //             // new_target = ((target as f32 / 26f32).ceil() as i64) - digit;
    //             // println!("ALT TARGET:{}", new_target);
    //             // test_wxyz = vec![0, 0, 0, new_target];
    //             // run_one(&input_stuff_vec[spot], &mut test_wxyz, digit);  // Run to get the total so far for the position.
    //             // println!("{} {} {:?}", new_num * zero_padding, digit, test_wxyz);
    //             // if test_wxyz[3] == target {
    //             //     if spot > limit {
    //             //         let ret_str = maybe_monad(&input_stuff_vec, &this_wxyz, new_num, new_target, spot - 1);
    //             //         if ret_str != "" {
    //             //             final_num += &ret_str;
    //             //         }
    //             //     }
    //             // }
    //         // }
    //         // if spot == limit && this_wxyz[3] == 0 {
    //         //     println!("MAX VALID!");
    //         //     final_num = (new_num * zero_padding).to_string();
    //         //     break;
    //         // }
    //     }
    //     return final_num;
    // }


    // #[allow(dead_code)]
    // fn reverse_gen(input_stuff_vec: &Vec<Vec<Vec<&str>>>) {
    //     let mut all_valid: Vec<Vec<Vec<i64>>> = vec![vec![vec![0, 0]]];
    //     let mut next_matches: Vec<Vec<i64>> = vec![];
    //     // let mut possible_max: String = "".to_string();

    //     for pos in 0..14 {
    //         next_matches = vec![];
    //         for i in (1..=9).rev() {
    //             // for k in 0..26 {
    //             let mut k = 1;
    //             let mut matches = 0;
    //             'outer: loop {
    //                 let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //                 run_one(&input_stuff_vec[13 - pos], &mut wxyz, i);
    //                 if all_valid[pos].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //                     println!("{}: NUM:{} Z:{} {:?}", pos, i, k, wxyz);
    //                     next_matches.push(vec![i, k, wxyz[3]]);
    //                     matches += 1;
    //                     // if matches == 9 {
    //                         // possible_max += &i.to_string();
    //                         // break;
    //                     // }
    //                 }
    //                 k += 1;
    //                 if k > 456976 {
    //                     k = 0;
    //                     loop {
    //                         let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //                         run_one(&input_stuff_vec[13 - pos], &mut wxyz, i);
    //                         if all_valid[pos].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //                             println!("{}: NUM:{} Z:{} {:?}", pos, i, k, wxyz);
    //                             next_matches.push(vec![i, k, wxyz[3]]);
    //                             matches += 1;
    //                             // if matches == 9 {
    //                                 // possible_max += &i.to_string();
    //                                 // break 'outer;
    //                             // }
    //                         }
    //                         k -= 1;
    //                         if k < -456976 {
    //                             break 'outer;
    //                         }
    //                     }
    //                 }
    //             }
    //         } 
    //         if next_matches.is_empty() {
    //             println!("INCOMPLETE!");
    //             break;
    //         }
    //         all_valid.push(next_matches);
    //     }
    //     println!("{}: {:?}", all_valid.len(), all_valid[all_valid.len() - 1]);
    //     // println!("MAX: {}", possible_max);
    //     let mut look_for = &all_valid[all_valid.len() - 1][0];  // Seed with the first item in the last digit list.
    //     let mut final_string: String = look_for[0].to_string();
    //     for pos in (0..(all_valid.len() - 1)).rev() {  // Start with second to last and go backwards.
    //         let this_pos = &all_valid[pos];
    //         for digit in (0..this_pos.len()).rev() {
    //             if this_pos[digit][1] == look_for[2] {
    //                 final_string += &this_pos[digit][0].to_string();
    //                 look_for = &this_pos[digit];
    //                 break;
    //             }
    //         }
    //     }
    //     println!("FINAL: {}", final_string);
    // }
    // // reverse_gen(&input_stuff_vec);
    // // 8913319110



    // let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // run_one(&input_stuff_vec[0], &mut wxyz, 9);
    // run_one(&input_stuff_vec[1], &mut wxyz, 4);
    // // println!("{} {:?}", 5, wxyz);
    // // run_one(&input_stuff_vec[0], &mut wxyz, 2);
    // // run_one(&input_stuff_vec[1], &mut wxyz, 6);
    // // println!("{} {:?}", 6, wxyz);
    // println!("{:?}", wxyz);

    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //     run_one(&input_stuff_vec[0], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }
    // println!();
    // 'outer: for i in (1..=9).rev() {
    //     // for z in -15000..15000 {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, z];
    //         // let mut wxyz: Vec<i64> = vec![0, 0, 0, z];
    //         // let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //         run_one(&input_stuff_vec[1], &mut wxyz, i);
    //         // println!("i:{} {:?}", i, wxyz);
    //         // println!("i:{} z:{} {:?}", i, z, wxyz);
    //         if wxyz[3] == 1 {
    //             println!("FOUND! i:{} z:{} {:?}", i, z, wxyz);
    //             break 'outer;
    //         }
    //     // }
    // }

    // println!();
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 7];
    //     run_one(&input_stuff_vec[2], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }

    // println!();
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, -14];
    //     run_one(&input_stuff_vec[2], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }


    // 9 -- z15-23 or z399-615
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //     run_one(&input_stuff_vec[10], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }

    // 10 -- z15-23 or z399-615
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 615];
    //     run_one(&input_stuff_vec[10], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }
    // 11 -- z10376-16000 or 390-623 or
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 623];
    //     run_one(&input_stuff_vec[11], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }
    // 12 -- needs z399-615
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 399];
    //     run_one(&input_stuff_vec[12], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }
    // 13 -- needs z15-23
    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 23];
    //     run_one(&input_stuff_vec[13], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }


    // next_matches = vec![];
    // // Can be 1-9. Need z to come in between 399 and 615.
    // for i in (1..=9).rev() {
    //     // for k in 0..26 {
    //     let mut k = 1;
    //     let mut matches = 0;
    //     loop {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //         run_one(&input_stuff_vec[12], &mut wxyz, i);
    //         if all_valid[1].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //             println!("{} {} {:?}", i, k, wxyz);
    //             next_matches.push(vec![i, k]);
    //             matches += 1;
    //             if matches == 9 {
    //                 break;
    //             }
    //         }
    //         k += 1;
    //         if k > 50000 {
    //             break;
    //         }
    //     }
    // } 
    // all_valid.push(next_matches);

    // next_matches = vec![];
    // for i in (1..=9).rev() {
    //     // for k in 0..26 {
    //     let mut k = 1;
    //     let mut matches = 0;
    //     loop {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //         run_one(&input_stuff_vec[11], &mut wxyz, i);
    //         if all_valid[2].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //             println!("{} {} {:?}", i, k, wxyz);
    //             next_matches.push(vec![i, k]);
    //             matches += 1;
    //             if matches == 9 {
    //                 break;
    //             }
    //         }
    //         k += 1;
    //         if k > 50000 {
    //             break;
    //         }
    //     }
    // } 
    // all_valid.push(next_matches);

    // next_matches = vec![];
    // // Can be 1-9. Need z to come in between 399 and 615.
    // for i in (1..=9).rev() {
    //     // for k in 0..26 {
    //     let mut k = 1;
    //     let mut matches = 0;
    //     loop {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //         run_one(&input_stuff_vec[10], &mut wxyz, i);
    //         if all_valid[2].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //             println!("{} {} {:?}", i, k, wxyz);
    //             next_matches.push(vec![i, k]);
    //             matches += 1;
    //             if matches == 9 {
    //                 break;
    //             }
    //         }
    //         k += 1;
    //         if k > 50000 {
    //             break;
    //         }
    //     }
    // } 
    // all_valid.push(next_matches);

    // next_matches = vec![];
    // // Can be 1-9. Need z to come in between 399 and 615.
    // for i in (1..=9).rev() {
    //     // for k in 0..26 {
    //     let mut k = 1;
    //     let mut matches = 0;
    //     loop {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, k];
    //         run_one(&input_stuff_vec[9], &mut wxyz, i);
    //         if all_valid[3].iter().map(|x| x[1]).collect::<Vec<i64>>().contains(&wxyz[3]) {
    //             println!("{} {} {:?}", i, k, wxyz);
    //             next_matches.push(vec![i, k]);
    //             matches += 1;
    //             if matches == 9 {
    //                 break;
    //             }
    //         }
    //         k += 1;
    //         if k > 50000 {
    //             break;
    //         }
    //     }
    // } 
    // all_valid.push(next_matches);


    // let mut ins_num = 0;
    // input_stuff_vec.iter().for_each(|ins_set| {
    //     println!("{}", ins_num);
    //     for x in (1..=9).rev() {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //         run_one(&ins_set, &mut wxyz, x);
    //         println!("{}: {:?}", x, wxyz);
    //         if wxyz[3] == 0 {
    //             println!("MAX VALID! {}", &x.to_string());
    //             break;
    //         }
    //     }
    //     // Display.
    //     ins_num += 1;
    // });


    // let mut final_num: String = "".to_string();
    // let wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // let final_num: String = next_digit(&input_stuff_vec, &wxyz, 0, 0 as usize);
    // println!("FINAL: {}", final_num);

    // let mut final_num: String = "".to_string();
    // let wxyz: Vec<i64> = vec![0, 0, 0, 615];
    // let final_num: String = next_digit(&input_stuff_vec, &wxyz, 0, 10 as usize);
    // println!("FINAL: {}", final_num);

    // let mut final_num: String = "".to_string();
    // let wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // let final_num: String = prev_digit(&input_stuff_vec, &wxyz, 0, 13 as usize);
    // println!("FINAL: {}", final_num);


    // let wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // let final_num: String = maybe_monad(&input_stuff_vec, &wxyz, 0, 0, 13 as usize);
    // println!("FINAL: {}", final_num);


    // for i in (1..=9).rev() {
    //     let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //     run_one(&input_stuff_vec[13], &mut wxyz, i);
    //     println!("{} {:?}", i, wxyz);
    // }
    // let mut new_z = 0;
    // loop {
    //     for i in (1..=9).rev() {
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, new_z];
    //         run_one(&input_stuff_vec[13], &mut wxyz, i);
    //         println!("{} {} {:?}", new_z, i, wxyz);
    //         if wxyz[3] == 0 {
    //             println!("MATCH!");
    //             break;
    //         }
    //     }
    //     new_z += 1;
    // }


    // let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // 'outer: loop {
    //     for x in (11111111111111 as i64..99999999999999 as i64).rev() {
    //         println!("{}:", x);
    //         let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    //         let x_vec: Vec<i64> = x.to_string().chars().map(|num| num.to_string().parse::<i64>().unwrap()).collect();
    //         for (idx, digit) in x_vec.iter().enumerate() {
    //             run_one(&input_stuff_vec[idx], &mut wxyz, *digit);
    //             println!("\t{}: {:?}", digit, wxyz);
    //         }
    //         if wxyz[3] == 0 {
    //             println!("MAX VALID!");
    //             final_num += &x.to_string();
    //             break 'outer;
    //         }
    //     }
    // }
    // let mut wxyz: Vec<i64> = vec![0, 0, 0, 0];
    // 'outer: for a in 1..9.rev() {
    //     run_one(&input_stuff_vec[0], &mut wxyz, a*10000000000000);
    //     for b in 1..9.rev() {
    //         let mut b_wxyz = wxyz.clone();
    //         run_one(&input_stuff_vec[1], &mut b_wxyz, ((a*10) + b) * 1000000000000);
    //         for c in 1..9.rev() {
    //             let mut c_wxyz = wxyz.clone();
    //             run_one(&input_stuff_vec[2], &mut wxyz, ((a*100) + (b*10) + c) * 100000000000);
    //             for d in 1..9.rev() {
    //                 let mut d_wxyz = wxyz.clone();
    //                 run_one(&input_stuff_vec[3], &mut wxyz, ((a*1000) + (b*100) + (c*10) + d) * 10000000000);
    //                 for e in 1..9.rev() {
    //                     let mut e_wxyz = wxyz.clone();
    //                     run_one(&input_stuff_vec[4], &mut wxyz, ((a*10000) + (b*1000) + (c*100) + (d*10) + e) * 1000000000);
    //                     for f in 1..9.rev() {
    //                         let mut f_wxyz = wxyz.clone();
    //                         run_one(&input_stuff_vec[5], &mut wxyz, ((a*100000) + (b*10000) + (c*1000) + (d*100) + (e*10) + f) * 100000000);
    //                         for g in 1..9.rev() {
    //                             let mut g_wxyz = wxyz.clone();
    //                             run_one(&input_stuff_vec[6], &mut wxyz, ((a*1000000) + (b*100000) + (c*10000) + (d*1000) + (e*100) + (f*10) + g) * 10000000);
    //                             for h in 1..9.rev() {
    //                                 let mut h_wxyz = wxyz.clone();
    //                                 run_one(&input_stuff_vec[7], &mut wxyz, ((a*10000000) + (b*1000000) + (c*100000) + (d*10000) + (e*1000) + (f*100) + (g*10) + h) * 1000000);
    //                                 for i in 1..9.rev() {
    //                                     run_one(&input_stuff_vec[8], &mut wxyz, ((a*100000000) + (b*10000000) + (c*1000000) + (d*100000) + (e*10000) + (f*1000) + (g*100) + (h*10) + i) * 100000);
    //                                     for j in 1..9.rev() {
    //                                         run_one(&input_stuff_vec[9], &mut wxyz, ((a*1000000000) + (b*100000000) + (c*10000000) + (d*1000000) + (e*100000) + (f*10000) + (g*1000) + (h*100) + (i*10) + j) * 10000);
    //                                         for k in 1..9.rev() {
    //                                             run_one(&input_stuff_vec[10], &mut wxyz, ((a*10000000000) + (b*1000000000) + (c*100000000) + (d*10000000) + (e*1000000) + (f*100000) + (g*10000) + (h*1000) + (i*100) + (j*10) + k) * 1000);
    //                                             for l in 1..9.rev() {
    //                                                 run_one(&input_stuff_vec[11], &mut wxyz, ((a*100000000000) + (b*10000000000) + (c*1000000000) + (d*100000000) + (e*10000000) + (f*1000000) + (g*100000) + (h*10000) + (i*1000) + (j*100) + (k*10) + l) * 100);
    //                                                 for m in 1..9.rev() {
    //                                                     run_one(&input_stuff_vec[12], &mut wxyz, ((a*1000000000000) + (b*100000000000) + (c*10000000000) + (d*1000000000) + (e*100000000) + (f*10000000) + (g*1000000) + (h*100000) + (i*10000) + (j*1000) + (k*100) + (l*10) + m) * 10);
    //                                                     for n in 1..9.rev() {
    //                                                         run_one(&input_stuff_vec[13], &mut wxyz, (a*10000000000000) + (b*1000000000000) + (c*100000000000) + (d*10000000000) + (e*1000000000) + (f*100000000) + (g*10000000) + (h*1000000) + (i*100000) + (j*10000) + (k*1000) + (l*100) + (m*10) + n;
    //                                                         println!("\t{}: {:?}", digit, wxyz);
    //                                                         if wxyz[3] == 0 {
    //                                                             println!("MAX VALID!");
    //                                                             final_num += &x.to_string();
    //                                                             break 'outer;
    //                                                         }
    //                                                     }
    //                                                 }
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
