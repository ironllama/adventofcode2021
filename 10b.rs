pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "[({(<(())[]>[[{[]{<()<>>",
    //     "[(()[<>])]({[<{<<[]>>(",
    //     "{([(<{}[<>[]}>{[]{[(<()>",
    //     "(((({<>}<{<{<>}{[]{[]{}",
    //     "[[<[([]))<([[{}[[()]]]",
    //     "[{[{({}]{}}([{[{{{}}([]",
    //     "{<[[]]>}<{[{[{[]{()[[[]",
    //     "[<(<(<(<{}))><([]([]()",
    //     "<{([([[(<>()){}]>(<<{{",
    //     "<{([{{}}[<[[[<>{}]]]>[]]",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("10.in.txt");

    let open_brackets = vec!['(', '[', '{' , '<'];
    let mut all_points = vec![];
    for line in input_stuff {
        let mut brackets = vec![];
        let mut corrupted = false;  // Add to track if each line is a corrupted line or not.
        let line_chars: Vec<char> = line.chars().collect();
        for this_char in line_chars {
            if open_brackets.contains(&this_char) {
                brackets.push(this_char);
            }
            else {
                let last_bracket = brackets.pop().unwrap();
                let matching_bracket = match last_bracket {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => ' ',
                };
                // println!("CHECK: {}", matching_bracket);
                if this_char != matching_bracket {
                    // corrupted.push(vec![line.to_string(), this_char.to_string()]);
                    corrupted = true;
                    break;
                }
            }
        }

        // Scoring done for each line.
        if !corrupted {
            brackets.reverse();  // Reverse the stack so I can iterate from the beginning. I guess you could also loop backwards.
            // println!("LEFT: {:?}", brackets);
            let mut points = 0;
            for to_close in brackets {
                let open_pos = open_brackets.iter().position(|x| *x == to_close).unwrap();
                points = (points * 5) + (open_pos + 1);
                // println!("B: {} POS: {} POINTS: {}", to_close, open_pos + 1, points);
            }

            all_points.push(points);
        }
    }
    // println!("CORR: {:?}", all_points);

    all_points.sort();
    let mid = all_points.len() / 2;
    println!("MID: {}", all_points[mid]);
}