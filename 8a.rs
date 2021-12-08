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

    // First just reduce into vector of tokens, but then realized I could count while processing each line, so unused.
    // let input_stuff_vec: Vec<&str> = input_stuff.iter().fold(vec![], |mut acc, x| {
    //     let mut tokens: Vec<&str> = x.split(" | ").collect::<Vec<&str>>()[1].split(' ').collect();
    //     acc.append(&mut tokens);
    //     acc
    // });

    // Process per line of input.
    let input_stuff_vec: i32 = input_stuff.iter().fold(0, |mut outer_acc, outer_x| {
        // Only use second section and split into word tokens.
        let mut tokens: Vec<&str> = outer_x.split(" | ").collect::<Vec<&str>>()[1].split(' ').collect();

        // Reduce by total number of words that are either 2, 3, 4 or 7 in length.
        let new_acc = tokens.iter().fold(0, |mut inner_acc, inner_x| {
            let num_chars: usize = inner_x.chars().count();
            if num_chars == 2  // Digit 1 on seven segment display
                || num_chars == 3  // Digit 7
                || num_chars == 4  // Digit 4
                || num_chars == 7  // Digit 8
            {
                inner_acc += 1;
            }
            inner_acc
        });

        // Add to the accumulator.
        outer_acc += new_acc;
        outer_acc
    });

    println!("TOTAL: {}", input_stuff_vec);
}