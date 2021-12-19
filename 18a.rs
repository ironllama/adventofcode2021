pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     // "[[[[4,3],4],4],[7,[[8,4],9]]]",
    //     // "[1,1]",
    //     // "[2,2]",
    //     // "[3,3]",
    //     // "[4,4]",
    //     // "[5,5]",
    //     // "[6,6]",
    //     // "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
    //     // "[2,9]",
    //     // "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
    //     // "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    //     // "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
    //     // "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
    //     // "[7,[5,[[3,8],[1,4]]]]",
    //     // "[[2,[2,2]],[8,[8,1]]]",
    //     // "[2,9]",
    //     // "[1,[[[9,3],9],[[9,0],[0,7]]]]",
    //     // "[[[5,[7,4]],7],1]",
    //     // "[[[[4,2],2],6],[8,7]]",
    //     // "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    //     "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    //     "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    //     "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    //     "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    //     "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    //     "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    //     "[[[[5,4],[7,7]],8],[[8,3],8]]",
    //     "[[9,3],[[9,9],[6,[4,9]]]]",
    //     "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    //     "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("18.in.txt");

    // Explode!
    fn reduce_brackets(in_line: &String) -> String {
        let in_line_vec: Vec<char> = in_line.chars().collect();
        let mut new_vec: Vec<char> = vec![];
        let mut num_brackets: i32 = 0;
        let mut last_num_idx: usize = 0;
        let mut last_num_len: usize = 0;

        let mut idx = 0;
        while idx < in_line_vec.len() {
            // println!("CURR: {} {:?}", in_line_vec[idx], new_vec);
            if in_line_vec[idx] == '[' {
                num_brackets += 1;

                if num_brackets > 4 {  // This is where the good stuff happens.
                    idx += 1;  // First char after the '['
                    let mut num_buff_left: Vec<char> = vec![in_line_vec[idx]];
                    let mut next_char = in_line_vec[idx + 1];  // Char after the first char. Test if 2 digit number. Assuming highest is 2 digit numbers!
                    if next_char != '[' && next_char != ']' && next_char != ',' {
                        num_buff_left.push(next_char);
                        idx += 1;
                    }
                    // println!("BUFF_LEFT: {:?}", num_buff_left);
                    let first_num_num = num_buff_left.iter().collect::<String>().parse::<i32>().unwrap();

                    idx += 2;  // Past comma
                    let mut num_buff_right: Vec<char> = vec![in_line_vec[idx]];  // The second number starts after idx + comma.
                    next_char = in_line_vec[idx + 1];  // Test if its a 2 digit number.
                    if next_char != '[' && next_char != ']' && next_char != ',' {
                        num_buff_right.push(next_char);
                        idx += 1;
                    }
                    // println!("BUFF_RIGHT: {:?}", num_buff_right);
                    let second_num_num = num_buff_right.iter().collect::<String>().parse::<i32>().unwrap();
                    // // println!("5 BRACKET NUMS: {:?} {:?} {:?}", first_num_num, add_to_next_num, num_buff_left);

                    if last_num_idx > 0 {  // Replace to the left of the first number... if there is a number before the first number.
                        let before_num_num: i32 = new_vec[last_num_idx..(last_num_idx + last_num_len)].iter().collect::<String>().parse::<i32>().unwrap();
                        let replacement: Vec<char> = (before_num_num + first_num_num).to_string().chars().collect();
                        // println!("5 BRACKET ADD PREVIOUS: {} {} {:?}", before_num_num, first_num_num, replacement);
                        new_vec.splice(last_num_idx..(last_num_idx + last_num_len), replacement);  // Replace the number before the first number.
                    }

                    new_vec.push('0');  // Replace the pair in the brackets with a '0'.

                    // Replace the number to the right of the second number. First, gotta find it.
                    idx += 2;  // Get past the ']'
                    let mut rest_of_line = in_line_vec[idx..].to_vec();  // Everything past the pair in brackets.
                    let pos_num: usize = match rest_of_line.iter().position(|x| *x != '[' && *x != ']' && *x != ',') {  // Find a number to the right!
                        Some(x) => x,
                        None => 0,
                    };
                    if pos_num > 0 {  // If a number was found.
                        let mut num_buff_after: Vec<char> = vec![rest_of_line[pos_num]];
                        next_char = rest_of_line[pos_num + 1];  // In case it is a 2 digit number.
                        if next_char != '[' && next_char != ']' && next_char != ',' {
                            num_buff_after.push(next_char);
                        }
                        let num_after = num_buff_after.iter().collect::<String>().parse::<i32>().unwrap();
                        let num_after_replace: Vec<char> = (num_after + second_num_num).to_string().chars().collect();
                        rest_of_line.splice(pos_num..(pos_num + num_buff_after.len()), num_after_replace);  // Replace the nmber after the second number.
                    }
                    new_vec.append(&mut rest_of_line);
                    // println!("END EXPLODE: {:?}", new_vec.iter().collect::<String>());

                    return new_vec.iter().collect::<String>();  // Return after processing at the 5th level of brackets.
                }
                else {  // Less than 5 brackets so far... just keep passing it on.
                    new_vec.push(in_line_vec[idx]);
                }
            }
            else if in_line_vec[idx] == ']' || in_line_vec[idx] == ',' {  // Just pass these on.
                if in_line_vec[idx] == ']' {
                    num_brackets -= 1;
                }
                new_vec.push(in_line_vec[idx]);
            }
            else {  // Most likely a number!
                let mut num_buff: Vec<char> = vec![in_line_vec[idx]];
                let next_char = in_line_vec[idx + 1];  // In case it is a 2 digit number.
                if next_char != '[' && next_char != ']' && next_char != ',' {
                    num_buff.push(next_char);
                    idx += 1;
                }
                let this_num: i32 = num_buff.iter().collect::<String>().parse::<i32>().unwrap();

                last_num_len = num_buff.len();  // To save from having to calculate it again later, when using it.
                // println!("ADD NUM FROM LAST 5 BRACKET: {} {} {:?}", this_num, add_to_next_num, new_num);
                new_vec.append(&mut this_num.to_string().chars().collect::<Vec<char>>());
                // println!("NEW_NUM_LEN: {} {:?}", new_num_len, (this_num + add_to_next_num));
                // println!("NEW_NUM_LEN: {} {:?}", new_num_len, this_num);
                
                last_num_idx  = new_vec.len() - last_num_len;  // To save from having to search the location of the last number.
                // println!("LAST NUM: {}", new_vec[last_num_idx]);
            }

            idx += 1;
        }

        new_vec.iter().collect()    // Returns if none at the 5th level of brackets.
    }

    // Split!
    fn reduce_bignums(in_line: &String) -> String {
        let in_line_vec: Vec<char> = in_line.chars().collect();
        let mut new_vec: Vec<char> = vec![];

        let mut idx = 0;
        while idx < in_line_vec.len() {
            let this_char = in_line_vec[idx];

            // Look for a number to process.
            if this_char != '[' && this_char != ']' && this_char != ',' {
                let mut num_buff: Vec<char> = vec![this_char];
                let next_char = in_line_vec[idx + 1];
                if next_char != '[' && next_char != ']' && next_char != ',' {  // Is the next character a second digit?
                    num_buff.push(next_char);
                }
                let new_num = num_buff.iter().collect::<String>().parse::<i32>().unwrap();

                if new_num > 9 {  // Found a double digit number!
                    let half_val = new_num as f32 / 2 as f32;
                    let mut left_num: Vec<char> = half_val.floor().to_string().chars().collect::<Vec<char>>();
                    let mut right_num: Vec<char> = half_val.ceil().to_string().chars().collect::<Vec<char>>();

                    // Construct the replacement pair in brackets.
                    let mut new_chars: Vec<char> = vec!['['];
                    new_chars.append(&mut left_num);
                    new_chars.push(',');
                    new_chars.append(&mut right_num);
                    new_chars.push(']');
                    new_vec.append(&mut new_chars);

                    new_vec.extend(&in_line_vec[(idx + num_buff.len())..]);  // Stick it in.
                    return new_vec.iter().collect();  // Return early after finding one double digit number.
                }
                else {
                    new_vec.push(this_char);
                }
            }
            else {
                new_vec.push(this_char);
            }

            idx += 1;
        }

        new_vec.iter().collect()  // Return if found nothing.
    }

    // Loop through each line from the input and process them.
    let mut last_line: String = "".to_string();
    input_stuff.iter().for_each(|line| {
        if last_line != "" {  // Skips first line. We need two lines in order to add. Hacky?
            // Create the new bracket pair using two lines of input.
            let mut new_line = "[".to_string();
            new_line.push_str(&last_line);
            new_line.push_str(",");
            new_line.push_str(&line);
            new_line.push_str("]");
            // println!("POST-ADD: {:?}", new_line);

            while new_line != last_line {  // Run the brackets!
                last_line = new_line.clone();
                new_line = reduce_brackets(&new_line);  // Explode!

                if new_line == last_line {  // Only run if explode is finished. Explode has priority.
                    new_line = reduce_bignums(&new_line);  // Split!
                    // println!("POST-SPLIT: {:?}", new_line);
                }
            }
        }
        else {
            last_line = line.to_string();  // Stores first line!
        }
    });

    println!("FINAL: {:?}", last_line);  // Resulting line!


    // Score the line.
    let mut last_line_vec: Vec<char> = last_line.chars().collect();
    while last_line_vec.contains(&',') {  // Searching from left to right, looking for a comma.
        let mut pos_next_comma = last_line_vec.iter().position(|x| *x == ',').unwrap();
        // let before_comma = last_line_vec[pos_next_comma - 1];
        let mut after_comma = last_line_vec[pos_next_comma + 1];
        // println!("VEC: {:?} BC {} AC {}", last_line_vec.iter().collect::<String>(), before_comma, after_comma);

        let mut offset = pos_next_comma + 1;
        while after_comma == '[' {  // If there is an open bracket to the right of the comma, look for next comma. We need numbers on both sides of comma.
            let pos_next_comma_next = last_line_vec.iter().skip(offset).position(|x| *x == ',').unwrap();
            pos_next_comma = offset + pos_next_comma_next;
            after_comma = last_line_vec[pos_next_comma + 1];
            // println!("FIND {} {}", pos_next_comma, after_comma);
            offset = pos_next_comma + 1;
        }

        let pos_bracket_start = last_line_vec[0..pos_next_comma].iter().rposition(|x| *x == '[').unwrap();  // Find start of first number, by looking for an open bracket.
        let left_num_str: String = last_line_vec[(pos_bracket_start + 1)..pos_next_comma].iter().collect();

        let pos_bracket_end = last_line_vec.iter().skip(pos_next_comma + 1).position(|x| *x == ']').unwrap();  // Find end of second number, by looking for a closed bracket.
        let right_num_str: String = last_line_vec[(pos_next_comma + 1)..(pos_next_comma + 1 + pos_bracket_end)].iter().collect();
        // println!("LEFT: {} RIGHT: {}", left_num_str, right_num_str);

        let left_num = left_num_str.parse::<i32>().unwrap();
        let right_num = right_num_str.parse::<i32>().unwrap();
        let new_num = (3 * left_num) + (2 * right_num);  // As specified in the instructions.
        let new_num_vec: Vec<char> = new_num.to_string().chars().collect();

        last_line_vec.splice((pos_next_comma - left_num.to_string().len() - 1)..(pos_next_comma + right_num.to_string().len() + 2), new_num_vec);  // Replace the pair!
        // println!("LLC {:?}", last_line_vec.iter().collect::<String>());
    }

    println!("{:?}", last_line_vec.iter().collect::<String>().parse::<i32>().unwrap());
}


// [[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9, 5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]] - START
// [[[[4,  0  ],[5,0]],[[[4,5],[2,6]],[9, 5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  0  ,[7,6]],[9, 5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[15,5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]] - EXPLODE
// 1234       3 4   32 34           3 4    321 2   345
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[15,5]]],[10,[[  0  ,[11,3]],[[6,3],[8 ,8]]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[15,5]]],[10,[[ 11  ,   0  ],[[9,3],[8 ,8]]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[15,5]]],[10,[[ 11  ,   9  ],[  0  ,[11,8]]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[15,5]]],[10,[[ 11  ,   9  ],[ 11  ,   0  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  0  ],[[7,8],5]]],[10,[[ 11  ,   9  ],[ 11  ,   0  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  0  ,13]]],[10,[[ 11  ,   9  ],[ 11  ,   0  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  0  ,[6,7]]]],[10,[[ 11  ,   9  ],[ 11  ,   0  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[17,[[ 11  ,   9  ],[ 11  ,   0  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,9],[[ 11  ,   9  ],[ 11  ,   0  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,9],[[[5,6],   9  ],[ 11  ,   0  ]]]] - SPLIT
// 1234       3 4   32 34           3 4           321 23   2 345
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,14],[[ 0,   15  ],[ 11  ,   0  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 0,   15  ],[ 11  ,   0  ]]]] - SPLIT
// 1234       3 4   32 34           3 4           321 23  4   32 34
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 0,  [7,8]],[ 11  ,   0  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 7,    0  ],[ 19  ,   0  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 7,    0  ],[[9,10],   0  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 7,    9  ],[  0   ,  10  ]]]] - EXPLODE
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 7,    9  ],[  0   ,  [5,5]  ]]]] - SPLIT
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[8,[7,7]],[[ 7,    9  ],[  5   ,    0  ]]]] - EXPLODE



// [[[[7,6],[0,6]],[[[10,10],14],[14,0]]],[[2,[11,10]],[[0,8],[8,0]]]
// 1234   3 4   32 345
// [[[[7,6],[0,16]],[[  0   ,24],[14,0]]],[[2,[11,10]],[[0,8],[8,0]]]



// [[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]
// 12345
// [[[[  0,  [12,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]
// [[[[  12,    0  ],[[12,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]
// [[[[  12,   12  ],[   0  ,[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]
// 1234            3 4       5
// [[[[  12,   12  ],[   6  ,  0  ]],[[[14,7],[8,9]],[8,[8,1]]]],[2,9]]
// 1234            3 4            32 345
// [[[[  12,   12  ],[   6  ,  14  ]],[[  0  ,[15,9]],[8,[8,1]]]],[2,9]]
// [[[[  12,   12  ],[   6  ,  14  ]],[[  15  ,  0  ],[17,[8,1]]]],[2,9]]
// 1234            3 4             32 34            3 4   5
// [[[[  12,   12  ],[   6  ,  14  ]],[[  15  ,  0  ],[25,  0  ]]],[3,9]]


// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  0  ,13]]],[[5,10],[[  0  ,  20  ],[  0  ,   6  ]]]]
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  0  ,[6,7]]]],[[5,[5,5]],[[  0  ,  [10,10]  ],[  0  ,   6  ]]]]
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  0  ,  0  ]]],[[12,[5,5]],[[  0  ,  [10,10]  ],[  0  ,   6  ]]]]
// [[[[4,  0  ],[5,4]],[[  7  ,  7  ],[  6  ,  0  ]]],[[[6,6],[5,5]],[[  0  ,  [[5,5],[5,5]]  ],[  0  ,   6  ]]]]


// [[[[4,  5  ],  0  ],[[[4,5],[2,6]],[9, 5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]]
// [[[[4,  5  ],  4  ],[[  0  ,[7,6]],[9, 5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]]
// [[[[4,  5  ],  4  ],[[  7  ,  0  ],[15,5]]],[7 ,[[[3,7],[4 ,3]],[[6,3],[8 ,8]]]]]
// [[[[4,  5  ],  4  ],[[  7  ,  0  ],[15,5]]],[10,[[  0  ,[11,3]],[[6,3],[8 ,8]]]]]
// [[[[4,  5  ],  4  ],[[  7  ,  0  ],[15,5]]],[10,[[  11 ,   0  ],[[9,3],[8 ,8]]]]]
// [[[[4,  5  ],  4  ],[[  7  ,  0  ],[15,5]]],[10,[[  11 ,   9  ],[  0  ,[11,8]]]]]
// [[[[4,  5  ],  4  ],[[  7  ,  0  ],[15,5]]],[10,[[  11 ,   9  ],[  11 ,   0 ]]]]
