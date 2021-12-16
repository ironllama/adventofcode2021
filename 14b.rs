pub mod usrlib;

use std::collections::HashMap;

fn main() {
    let input_stuff = [
        "NNCB",
        "",
        "CH -> B",
        "HH -> N",
        "CB -> H",
        "NH -> C",
        "HB -> C",
        "HC -> B",
        "HN -> C",
        "NN -> C",
        "BH -> H",
        "NC -> B",
        "NB -> B",
        "BN -> B",
        "BB -> N",
        "BC -> B",
        "CC -> N",
        "CN -> C",
    ];
    // let input_stuff = usrlib::vec_lines_from_file("14.in.txt");

    let mut starting: Vec<char> = vec![];
    // let pairs: Vec<(&str, &str)> = input_stuff.iter().fold(vec![], |mut acc, line| {
    let pairs: HashMap<&str, &str> = input_stuff.iter().fold(HashMap::new(), |mut acc, line| {
        if !line.is_empty() {
            if line.contains("->") {
                let mut split_iter = line.split(" -> ");
                // acc.push((split_iter.next().unwrap(), split_iter.next().unwrap()));  // Tuples!
                acc.insert(split_iter.next().unwrap(), split_iter.next().unwrap());
            }
            else {
                starting = line.chars().collect();
            }
        }
        acc
    });
    // println!("{:?} {:?}", starting, pairs);

    // fn cycle(in_starting: &mut Vec<char>, pairs: &HashMap<&str, &str>) -> Vec<char> {
    //     let mut ending: Vec<char> = vec![];
    //     // let mut counts: HashMap<String, i32> = HashMap::new();
    //     for idx in 0..(in_starting.len() - 1) {
    //         let pair: String = format!("{}{}", in_starting[idx], in_starting[idx + 1]);
    //         match pairs.get(&pair as &str) {
    //             Some(x) => {
    //                     ending.push(in_starting[idx]);
    //                     ending.push(x.chars().next().unwrap());
    //                 },
    //             None => (),
    //         }
    //     }
    //     ending.push(*in_starting.iter().last().unwrap());

    //     return ending;
    // }
    fn cycle(in_starting: &mut String, pairs: &HashMap<&str, &str>) {
        let orig_starting = in_starting.clone();
        let mut new_string = "".to_string();
        for idx in 0..(orig_starting.len() - 1) {
            // let pair: String = format!("{}{}", in_starting[idx], in_starting[idx + 1]);
            let pair = &orig_starting[idx..(idx + 2)];
            // println!("PAIR: {:?}", pair);
            // let pair_0 = pair.chars().nth(0).unwrap().to_string();
            // let pair_1 = pair.chars().nth(1).unwrap().to_string();
            let pair_0 = &orig_starting[idx..(idx + 1)];
            match pairs.get(&pair as &str) {
                Some(x) => {
                        // ending.push(in_starting[idx]);
                        // ending.push(x.chars().next().unwrap());
                        // let replacement = [pair_0, x.to_string(), pair_1].concat();
                        // let new_string = in_starting.replace(&pair, &replacement);
                        // *in_starting = new_string;
                        new_string.push_str(pair_0);
                        new_string.push_str(*x);
                    },
                None => (),
            }
        }
        new_string.push_str(&orig_starting[orig_starting.len() - 1..]);
        *in_starting = new_string;
    }


    // Using percent chance of appearance.
    // let num_cycles = 40;

    // let pair_values: Vec<&str> = pairs.values().cloned().collect();
    // let num_b = pair_values.iter().fold(0, | mut acc, x | { if x == &"B" { acc += 1; } acc });
    // let per_b = num_b as f64 / pairs.len() as f64;
    // let num_h = pair_values.iter().fold(0, | mut acc, x | { if x == &"H" { acc += 1; } acc });
    // let per_h = num_h as f64 / pairs.len() as f64;
    // let num_c = pair_values.iter().fold(0, | mut acc, x | { if x == &"C" { acc += 1; } acc });
    // let per_c = num_c as f64 / pairs.len() as f64;
    // let num_n = pair_values.iter().fold(0, | mut acc, x | { if x == &"N" { acc += 1; } acc });
    // let per_n = num_n as f64 / pairs.len() as f64;
    // // println!("P_V: {:?} {} {}", pair_values, num_c, num_h);
    // println!("PERC: {} {} {} {}", num_b, per_b, num_h, per_h);

    // let mut length: i64 = 4;
    // println!("[0] L: {}", length);
    // for idx in 1..(num_cycles + 1) {  // Starting with 1 for the debug output.
    //     length += length - 1;
    //     println!("[{}] L: {} B: {} H: {} C:{} N:{}", idx, length, per_b * (length as f64), per_h * (length as f64), per_c * (length as f64), per_n * (length as f64));
    // }


    // let mut ending: Vec<char> = vec![];

    let num_cycles = 40;
    let mut working = starting.iter().collect();
    // println!("[{}] {}", 0, working.iter().collect::<String>());
    for idx in 1..(num_cycles + 1) {  // Starting with 1 for the debug output.
        // working = cycle(&mut working, &pairs);
        cycle(&mut working, &pairs);
        // println!("[{}] {:?}", idx, working);
        println!("[{}]", idx);

        let counts: HashMap<char, i32> = working.chars().fold(HashMap::new(), | mut acc, this_char | {
            match acc.get_mut(&this_char) {
                Some(x) => *x += 1,
                None => { acc.insert(this_char, 1); },
            }
            acc
        });
        // let count_b = counts.get(&'B').unwrap();
        // let count_h = counts.get(&'H').unwrap();
        // let count_c = counts.get(&'C').unwrap();
        // let count_n = counts.get(&'N').unwrap();
        // let working_size = working.len() as f64;
        // println!("B: {} {} H: {} {} C: {} {} N: {} {}",
        //     count_b, *count_b as f64 / working_size,
        //     count_h, *count_h as f64 / working_size,
        //     count_c, *count_c as f64 / working_size,
        //     count_n, *count_n as f64 / working_size
        // );

        // let mut all_scores: Vec<i32> = counts.values().cloned().collect();
        //all_scores.sort();
        // println!("{:?}", all_scores);
    }

    let counts: HashMap<char, i32> = working.chars().fold(HashMap::new(), | mut acc, this_char | {
        match acc.get_mut(&this_char) {
            Some(x) => *x += 1,
            None => { acc.insert(this_char, 1); },
        }
        acc
    });
    println!("{:?}", counts);
    let mut all_scores: Vec<i32> = counts.values().cloned().collect();
    all_scores.sort();
    // println!("{:?}", all_scores);
    println!("SCORE {:?}", all_scores[all_scores.len() - 1] - all_scores[0]);

}