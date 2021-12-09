pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    //     "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    //     "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    //     "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    //     "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    //     "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    //     "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    //     "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    //     "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    //     "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("8.in.txt");

    // First reduce into vector of vector of tokens - first element for left of | and second for right of |
    let input_stuff_vec: Vec<Vec<Vec<&str>>> = input_stuff.iter().fold(vec![], |mut acc, x| {
        let left_and_right: Vec<&str> = x.split(" | ").collect();
        let all_tokens: Vec<Vec<&str>> = left_and_right.iter().map(|line_part| line_part.split(' ').collect() ).collect();
        acc.push(all_tokens);
        acc
    });

    // Utility function to find words in a vector that have a certain length.
    fn get_words_with_num_chars<'a>(num_chars: i32, words: &'a Vec<&str>) -> Option<Vec<&'a str>> {
        let mut returnVec = vec![];
        for idx in 0..words.len() {
            // println!("LEN: {}", words[idx].chars().count());
            if words[idx].chars().count() == num_chars as usize {
                // return Some(&words[idx]);
                returnVec.push(words[idx]);
            }
        }
        // None
        if returnVec.len() > 0 {
            return Some(returnVec);
        }
        else {
            return None;
        }
    }

    // Get a number for each line in the input.
    let all_nums: Vec<i32> = input_stuff_vec.iter().fold(vec![], | mut acc, line | {
        let mut led_array = vec!['x'; 7];
        // println!("LINE: {:?}", line);
        let digit_one: Vec<char> = get_words_with_num_chars(2, &line[0]).unwrap()[0].chars().collect();
        let digit_seven: Vec<char> = get_words_with_num_chars(3, &line[0]).unwrap()[0].chars().collect();
        let digit_four: Vec<char> = get_words_with_num_chars(4, &line[0]).unwrap()[0].chars().collect();
        let digit_eight: Vec<char> = get_words_with_num_chars(7, &line[0]).unwrap()[0].chars().collect();
        

        // Assumed that the segments were given in order on the left set, so I used the 8 to get the signments. Of course, that would be too easy.
        // The segments given on the left set were in random order.
        // ====================================================================
        // let mut digit_eight = "";
        // for idx in 0..line[0].len() {
        //     if line[0][idx].len() == 7 {
        //         digit_eight = line[0][idx];
        //         break;
        //     }
        // }

        // if digit_eight != "" {
        //     // println!("8: {}", digit_eight);
        //     let digit_eight_chars: Vec<char> = digit_eight.chars().collect();
        //     println!("8: {:?}", digit_eight_chars);

        // So, then went through and logic-ed out the proper segments per number.
        // ====================================================================
        // let top: Vec<&char> = digit_seven[0].chars().collect::<Vec<char>>().iter().filter(|x| !digit_one[0].chars().collect::<Vec<char>>().contains(x) ).collect();
        let top: Vec<&char> = digit_seven.iter().filter(|x| !digit_one.contains(x)).collect();  // Determine the top segment comparing 1 and 7.
        led_array[0] = top[0].to_owned();
        // println!("TOP: {:?}", led_array[0]);

        let digit_six_or_nine_or_zero = get_words_with_num_chars(6, &line[0]).unwrap();  // Get all words that could be a 6, 9, or 0.
        // println!("6 or 9 or 0: {:?}", digit_six_or_nine_or_zero);
        for word in digit_six_or_nine_or_zero {
            let word_chars: Vec<char> = word.chars().collect();

            let top_right: Vec<&char> = digit_one.iter().filter(|x| !word_chars.contains(x)).collect();  // Compare the 1 and each number to find the 6.
            if top_right.len() == 1 {  // Found the 6!
                led_array[2] = top_right[0].to_owned();  // Left over segment is the top right.
                // println!("SIX: {:?}", led_array[2]);

                let bottom_right: Vec<&char> = digit_one.iter().filter(|x| word_chars.contains(x)).collect();  // The segment from 1 that didn't match above is bottom right.
                led_array[5] = bottom_right[0].to_owned();
                // println!("BOTTOMRIGHT: {:?}", bottom_right);
            }
            else {
                let digit_nine_str: Vec<&char> = digit_four.iter().filter(|x| !word_chars.contains(x)).collect();  // The 4 completely goes into a 9.
                // println!("NINE: {:?}", digit_nine_str);
                if digit_nine_str.len() == 0 {  // Found the 9!
                    // println!("FOUND: {}", word);
                    let mut digit_four_plus = digit_four.clone();
                    digit_four_plus.push(led_array[0]);  // Added the top to the 4 to get all but 1 segment of the 9.
                    let bottom: Vec<&char> = word_chars.iter().filter(|x| !digit_four_plus.contains(x)).collect();  // That leftover segment is the bottom.
                    led_array[6] = bottom[0].to_owned();

                    let bottom_left: Vec<&char> = digit_eight.iter().filter(|x| !word_chars.contains(x)).collect();  // Compare the 8 and 9 to get the bottom left.
                    led_array[4] = bottom_left[0].to_owned();
                    // println!("BOTTOMLEFT: {:?}", bottom_left);
                }
                else {  // Found the 0! It's not a 6 or a 9.
                    let middle: Vec<&char> = digit_eight.iter().filter(|x| !word_chars.contains(x)).collect();  // Compare 0 against the 8 to get the middle segment.
                    led_array[3] = middle[0].to_owned();
                    // println!("MIDDLE: {:?}", middle);
                }
            }
        }

        let top_left: Vec<&char> = digit_eight.iter().filter(|x| !led_array.contains(x)).collect();  // The unassignment segment is the top left.
        led_array[1] = top_left[0].to_owned();

        // println!("ALL SEGMENTS: {:?}", led_array);


        // Manually constructed what each number should have looked like.
        let mut digit_two: Vec<char> = vec![led_array[0], led_array[2], led_array[3], led_array[4], led_array[6]];
        let mut digit_three: Vec<char> = vec![led_array[0], led_array[2], led_array[3], led_array[5], led_array[6]];
        let mut digit_five: Vec<char> = vec![led_array[0], led_array[1], led_array[3], led_array[5], led_array[6]];
        let mut digit_six: Vec<char> = vec![led_array[0], led_array[1], led_array[3], led_array[4], led_array[5], led_array[6]];
        let mut digit_nine: Vec<char> = vec![led_array[0], led_array[1], led_array[2], led_array[3], led_array[5], led_array[6]];
        let mut digit_zero: Vec<char> = vec![led_array[0], led_array[1], led_array[2], led_array[4], led_array[5], led_array[6]];

        // Then sorted by segments, since the second set also had the segments in random order. Didn't figure this out until later. (See line 193.)
        digit_two.sort();
        digit_three.sort();
        digit_five.sort();
        digit_six.sort();
        digit_nine.sort();
        digit_zero.sort();

        // Get the actual numbers decoded from the second set.
        let new_acc: String = line[1].iter().fold("".to_string(), |mut inner_acc, inner_x| {
            let mut line_chars: Vec<char> = inner_x.chars().collect();
            let num_chars: usize = line_chars.len();

            if num_chars == 2 {  // Digit 1 on seven segment display
                inner_acc.push_str("1");
            }
            else if num_chars == 3 {  // Digit 7
                inner_acc.push_str("7");
            }
            else if num_chars == 4  {  // Digit 4
                inner_acc.push_str("4");
            }
            else if num_chars == 7 {  // Digit 8
                inner_acc.push_str("8");
            }
            else {
                // println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", line_chars, digit_two, digit_three, digit_five, digit_six, digit_nine, digit_zero);
                // println!("{} {}", inner_x.to_string(), digit_three);
            
                // First, tried just comparing lengths, but quickly discovered that some numbers can overshadow others. DOES NOT WORK.
                // let two_left: Vec<&char> = digit_two.iter().filter(|x| !line_chars.contains(x)).collect();
                // if two_left.len() == 0 {
                //     inner_acc.push_str("2");
                // }
                // else {
                //     let three_left: Vec<&char> = digit_three.iter().filter(|x| !line_chars.contains(x)).collect();
                //     if three_left.len() == 0 {
                //         inner_acc.push_str("3");
                //     }
                //     else {
                //         let five_left: Vec<&char> = digit_five.iter().filter(|x| !line_chars.contains(x)).collect();
                //         if five_left.len() == 0 {
                //             inner_acc.push_str("5");
                //         }
                //         else {
                //             let six_left: Vec<&char> = digit_six.iter().filter(|x| !line_chars.contains(x)).collect();
                //             if six_left.len() == 0 {
                //                 inner_acc.push_str("6");
                //             }
                //             else {
                //                 let nine_left: Vec<&char> = digit_nine.iter().filter(|x| !line_chars.contains(x)).collect();
                //                 if nine_left.len() == 0 {
                //                     inner_acc.push_str("9");
                //                 }
                //                 else {
                //                     let zero_left: Vec<&char> = digit_zero.iter().filter(|x| !line_chars.contains(x)).collect();
                //                     if zero_left.len() == 0 {
                //                         inner_acc.push_str("0");
                //                     }
                //                 }
                //             }
                //         }
                //     }
                // }

                // So, compared vectors, segment by segment. Because of the random order of segments in the input, sorted both the input and the deduced segments.
                line_chars.sort();
                // println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", line_chars, digit_two, digit_three, digit_five, digit_six, digit_nine, digit_zero);

                if digit_two == line_chars {
                    inner_acc.push_str("2");
                }
                else if digit_three == line_chars {
                    inner_acc.push_str("3");
                }
                else if digit_five == line_chars {
                    inner_acc.push_str("5");
                }
                else if digit_six == line_chars {
                    inner_acc.push_str("6");
                }
                else if digit_nine == line_chars {
                    inner_acc.push_str("9");
                }
                else if digit_zero == line_chars {
                    inner_acc.push_str("0");
                }

            }
            inner_acc
        });
        // println!("STR: {}", new_acc);

        acc.push(new_acc.parse::<i32>().unwrap());
        acc
    });

    println!("TOTAL: {:?}", all_nums.iter().sum::<i32>());
}