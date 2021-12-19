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

    fn get_score(last_line: &String) -> i32 {
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

        return last_line_vec.iter().collect::<String>().parse::<i32>().unwrap()
    }
    

    let mut highest_score = 0;
    let mut highest_line = "".to_string();
    // We need to test every line with every other line.
    for one in 0..(input_stuff.len() - 1) {
        for two in (one + 1)..input_stuff.len() {
            // First, test it one way, with one + two.
            let mut new_line = "[".to_string();
            new_line.push_str(&input_stuff[one]);
            new_line.push_str(",");
            new_line.push_str(&input_stuff[two]);
            new_line.push_str("]");
            let mut lines_added = new_line.clone();

            let mut last_line = "".to_string();
            while new_line != last_line {  // Run the brackets!
                last_line = new_line.clone();
                new_line = reduce_brackets(&new_line);

                if new_line == last_line {
                    new_line = reduce_bignums(&new_line);
                    // println!("POST-SPLIT: {:?}", new_line);
                }
            }

            let mut curr_score = get_score(&last_line);
            // println!("CURR: {:?} {:?}", curr_score, lines_added);
            if curr_score > highest_score {
                highest_score = curr_score;
                highest_line = lines_added;
            }


            // Then test it the other way, with two + one.
            new_line = "[".to_string();
            new_line.push_str(&input_stuff[two]);
            new_line.push_str(",");
            new_line.push_str(&input_stuff[one]);
            new_line.push_str("]");
            lines_added = new_line.clone();

            last_line = "".to_string();
            while new_line != last_line {  // Run the brackets!
                last_line = new_line.clone();
                new_line = reduce_brackets(&new_line);

                if new_line == last_line {
                    new_line = reduce_bignums(&new_line);
                    // println!("POST-SPLIT: {:?}", new_line);
                }
            }

            curr_score = get_score(&last_line);
            // println!("CURR: {:?} {:?}", curr_score, lines_added);
            if curr_score > highest_score {
                highest_score = curr_score;
                highest_line = lines_added;
            }
        }
    }

    println!("HIGHEST LINE: {:?}", highest_line);
    println!("HIGHEST SCORE: {:?}", highest_score);
}
