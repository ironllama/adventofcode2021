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
    let ops_lines = vec![4, 5, 15];

    let mut ops: Vec<Vec<i64>> = vec![vec![]; ops_lines.len()];
    for this_pos in 0..num_pos {
        for (idx, op_num) in ops_lines.iter().enumerate() {
            let offset = lines_per_section * this_pos;
            ops[idx].push(input_stuff[offset + op_num].split_whitespace().last().unwrap().parse().unwrap());
        }
    }
    println!("OPS: {:?}", ops);

    let mut stack: Vec<usize> = vec![0 as usize];
    let mut stack_iter: usize = 0;
    let mut final_num: Vec<i64> = vec![0; 14];
    while !stack.is_empty() {
        stack_iter += 1;
        if ops[0][stack_iter] == 26 {
            let idx_one = stack.pop().unwrap() as usize;
            let diff = ops[2][idx_one] + ops[1][stack_iter];
            // if diff < 0 {
            //     final_num[idx_one] = 9;
            //     final_num[stack_iter] = 9 + diff;
            // }
            // else {
            //     final_num[idx_one] = 9 - diff;
            //     final_num[stack_iter] = 9;
            // }

            // Same as part a, except, assigning 1's instead of 9's to opposite pairs.
            if diff < 0 {
                final_num[idx_one] = (diff * -1) + 1;
                final_num[stack_iter] = 1;
            }
            else {
                final_num[idx_one] = 1;
                final_num[stack_iter] = 1 + diff;
            }
        }
        else {
            stack.push(stack_iter);
        }
    }
    // println!("FINAL_NUM: {:?}", final_num);  // 99394899891971
    print!("FINAL NUM: ");
    final_num.iter().for_each(|x| print!("{}", x));
    println!();
}