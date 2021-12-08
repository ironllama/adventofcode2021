pub mod usrlib;

fn main() {
    let input_stuff = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];
    // let input_stuff = usrlib::vec_lines_from_file("8.in.txt");

    // First just reduce into vector of vector of tokens - first element for left of | and second for right of |
    let input_stuff_vec: Vec<Vec<Vec<&str>>> = input_stuff.iter().fold(vec![], |mut acc, x| {
        let left_and_right: Vec<&str> = x.split(" | ").collect();
        let all_tokens: Vec<Vec<&str>> = left_and_right.iter().map(|line_part| line_part.split(' ').collect() ).collect();
        acc.push(all_tokens);
        acc
    });

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

    for line in &input_stuff_vec {
        // println!("LINE: {:?}", line);
        let digit_one: Vec<char> = get_words_with_num_chars(2, &line[0]).unwrap()[0].chars().collect();
        let digit_seven: Vec<char> = get_words_with_num_chars(3, &line[0]).unwrap()[0].chars().collect();
        println!("1: {:?}", digit_one);
        println!("7: {:?}", digit_seven);

        let top: Vec<&char> = digit_seven.iter().filter(|x| !digit_one.contains(x) ).collect();
        println!("TOP: {:?}", top);

        let digit_six_or_nine_or_zero = get_words_with_num_chars(6, &line[0]).unwrap();
        println!("6 or 9 or 0: {:?}", digit_six_or_nine_or_zero);


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
        //     let digit_two: String = vec![digit_eight_chars[0], digit_eight_chars[2], digit_eight_chars[3], digit_eight_chars[4], digit_eight_chars[6]].iter().collect();
        //     let digit_three: String = vec![digit_eight_chars[0], digit_eight_chars[2], digit_eight_chars[3], digit_eight_chars[5], digit_eight_chars[6]].iter().collect();
        //     let digit_five: String = vec![digit_eight_chars[0], digit_eight_chars[1], digit_eight_chars[3], digit_eight_chars[4], digit_eight_chars[6]].iter().collect();
        //     let digit_six: String = vec![digit_eight_chars[0], digit_eight_chars[2], digit_eight_chars[3], digit_eight_chars[4], digit_eight_chars[5], digit_eight_chars[6]].iter().collect();
        //     let digit_nine: String = vec![digit_eight_chars[0], digit_eight_chars[1], digit_eight_chars[2], digit_eight_chars[3], digit_eight_chars[5], digit_eight_chars[6]].iter().collect();
        //     // println!("2: {:?}", digit_two);
        //     // println!("3: {:?}", digit_three);
        //     // println!("5: {:?}", digit_five);
        //     // println!("6: {:?}", digit_six);
        //     // println!("9: {:?}", digit_nine);


        //     let new_acc: String = line[1].iter().fold("".to_string(), |mut inner_acc, inner_x| {
        //         let num_chars: usize = inner_x.chars().count();
        //         if num_chars == 2 {  // Digit 1 on seven segment display
        //             inner_acc.push_str("1");
        //         }
        //         else if num_chars == 3 {  // Digit 7
        //             inner_acc.push_str("7");
        //         }
        //         else if num_chars == 4  {  // Digit 4
        //             inner_acc.push_str("4");
        //         }
        //         else if num_chars == 7 {  // Digit 8
        //             inner_acc.push_str("8");
        //         }
        //         else {
        //             // println!("{} {}", inner_x.to_string(), digit_two);
        //             println!("{} {}", inner_x.to_string(), digit_three);
        //             // println!("{} {}". inner_x.to_string(), digit_five);
        //             // println!("{} {}". inner_x.to_string(), digit_two);
        //             // println!("{} {}". inner_x.to_string(), digit_two);
        //             if inner_x.to_string() == digit_two {
        //                 inner_acc.push_str("2");
        //             }
        //             else if inner_x.to_string() == digit_three {
        //                 inner_acc.push_str("3");
        //             }
        //             else if inner_x.to_string() == digit_five {
        //                 inner_acc.push_str("5");
        //             }
        //             else if inner_x.to_string() == digit_six {
        //                 inner_acc.push_str("6");
        //             }
        //             else if inner_x.to_string() == digit_nine {
        //                 inner_acc.push_str("9");
        //             }
        //         }
        //         inner_acc
        //     });
        //     println!("STR: {}", new_acc);
        // }
        // else {
        //     println!("8: NONE!");
        // }
    }


    println!("TOTAL: {:?}", input_stuff_vec);
}