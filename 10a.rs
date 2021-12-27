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
    let mut corrupted = vec![];  // Final list of corrupted lines.

    for line in input_stuff {
        let mut brackets = vec![];  // This is the stack to store all the open brackets, as the line is processed.
        let line_chars: Vec<char> = line.chars().collect();
        for this_char in line_chars {
            if open_brackets.contains(&this_char) {  // Open bracket!
                brackets.push(this_char);  // Just push and continue.
            }
            else {  // Closed bracket.
                let last_bracket = brackets.pop().unwrap();  // Get last open bracket.
                let matching_bracket = match last_bracket {  // Get the "expected" closing bracket for opening bracket.
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => ' ',
                };
                // println!("CHECK: {}", matching_bracket);
                if this_char != matching_bracket {  // Check for match.
                    corrupted.push(vec![line.to_string(), this_char.to_string()]);
                    break;
                }
            }
        }
    }
    // println!("CORR: {:?}", corrupted);
    // for line in &corrupted {
    //     println!("CORR: {:?}", line);
    // }

    // Scoring.
    // let closed_brackets = vec![[')', 3], [']', 57], ['}', 1197] , ['>', 25137]];  // Meh. Would have to iterate/break, rather than match.
    let mut total_points = 0;
    for line in &corrupted {
        // let corrupted_points = corrupted.iter.map(|line| closed_brackets.iter().filter(|x| x == line[1])
        // total_points += corrupted[]
        let corrupted_points = match line[1].as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => 0,
        };
        total_points += corrupted_points;
    }
    println!("TOTAL: {}", total_points);
}