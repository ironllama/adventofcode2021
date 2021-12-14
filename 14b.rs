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

    fn cycle(in_starting: &mut Vec<char>, pairs: &HashMap<&str, &str>) -> Vec<char> {
        let mut ending: Vec<char> = vec![];
        // let mut counts: HashMap<String, i32> = HashMap::new();
        for idx in 0..(in_starting.len() - 1) {
            let pair: String = format!("{}{}", in_starting[idx], in_starting[idx + 1]);
            match pairs.get(&pair as &str) {
                Some(x) => {
                        ending.push(in_starting[idx]);
                        ending.push(x.chars().next().unwrap());
                    },
                None => (),
            }
        }
        ending.push(*in_starting.iter().last().unwrap());

        return ending;
    }

    // let mut ending: Vec<char> = vec![];

    let num_cycles = 10;
    let mut working = starting.clone();
    // println!("[{}] {}", 0, working.iter().collect::<String>());
    for idx in 1..(num_cycles + 1) {  // Starting with 1 for the debug output.
        working = cycle(&mut working, &pairs);
        // println!("[{}] {:?}", idx, working.iter().collect::<String>());
        println!("[{}]", idx);

        let counts: HashMap<char, i32> = working.iter().fold(HashMap::new(), | mut acc, this_char | {
            match acc.get_mut(&this_char) {
                Some(x) => *x += 1,
                None => { acc.insert(*this_char, 1); },
            }
            acc
        });
        println!("{:?}", counts);
        let mut all_scores: Vec<i32> = counts.values().cloned().collect();
        all_scores.sort();
        // println!("{:?}", all_scores);
    }

    let counts: HashMap<char, i32> = working.iter().fold(HashMap::new(), | mut acc, this_char | {
        match acc.get_mut(&this_char) {
            Some(x) => *x += 1,
            None => { acc.insert(*this_char, 1); },
        }
        acc
    });
    println!("{:?}", counts);
    let mut all_scores: Vec<i32> = counts.values().cloned().collect();
    all_scores.sort();
    // println!("{:?}", all_scores);
    println!("SCORE {:?}", all_scores[all_scores.len() - 1] - all_scores[0]);

}