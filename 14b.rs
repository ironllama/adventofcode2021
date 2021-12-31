pub mod usrlib;

use std::collections::HashMap;

fn main() {
    // let input_stuff = [
    //     "NNCB",
    //     "",
    //     "CH -> B",
    //     "HH -> N",
    //     "CB -> H",
    //     "NH -> C",
    //     "HB -> C",
    //     "HC -> B",
    //     "HN -> C",
    //     "NN -> C",
    //     "BH -> H",
    //     "NC -> B",
    //     "NB -> B",
    //     "BN -> B",
    //     "BB -> N",
    //     "BC -> B",
    //     "CC -> N",
    //     "CN -> C",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("14.in.txt");

    // Init data into HashMap of string pairs.
    let mut starting: Vec<char> = vec![];
    let pairs: HashMap<&str, &str> = input_stuff.iter().fold(HashMap::new(), |mut acc, line| {
        if !line.is_empty() {
            if line.contains("->") {
                let mut split_iter = line.split(" -> ");
                acc.insert(split_iter.next().unwrap(), split_iter.next().unwrap());
            }
            else {
                starting = line.chars().collect();
            }
        }
        acc
    });
    // println!("{:?} {:?}", starting, pairs);

    fn display_sorted(this_hash: &HashMap<String, i64>) {
        let mut lookup_keys: Vec<&String> = this_hash.keys().collect();
        lookup_keys.sort();
        let mut total = 0;
        lookup_keys.iter().for_each(|&vk| { print!("{}:{}, ", vk, this_hash.get(vk).unwrap()); total += this_hash.get(vk).unwrap(); });
        print!("TOTAL:{}", total);
        println!();
    }

    // Create a counter for each letter. Some may not yet be used in the starting sequence.
    let mut count_chars: HashMap<String, i64> = starting.iter().fold(HashMap::new(), |mut acc, x| { *acc.entry(x.to_string()).or_default() += 1; acc });
    // display_sorted(&count_chars);

    // Create a frequency counting table for pairs. To help track how much of each to grow out each generation.
    let mut count_pairs: HashMap<String, i64> = pairs.keys().fold(HashMap::new(), |mut acc, pairs_key| { acc.insert(pairs_key.to_string(), 0); acc });
    // display_sorted(&count_pairs);

    // Seed the count_pairs hashmap with the letters from the starting line.
    starting.windows(2).for_each(|pair_letters| *count_pairs.entry(pair_letters.iter().collect::<String>()).or_default() += 1);
    // display_sorted(&count_pairs);

    // Run the number of cycles required. Run lots of counting as needed, rather than generating all the strings.
    // let limit = 10;
    let limit = 40;
    for _cycle in 1..=limit {
        // println!("CYCLE: {}", cycle);
        count_pairs = count_pairs.iter().fold(HashMap::new(), |mut acc, (key, value)| {  // For each of the existing pairs. Use a new copy to accumulate into.
            let new_char: String = pairs.get(&key as &str).unwrap().to_string();
            let one: String = key[..1].to_string() + &new_char as &str;
            let two: String = new_char.clone() + &key[1..];
            // println!("CYCLE:{} KEY:{} VAL:{} NEW:{} ONE:{} TWO:{}", cycle, key, value, new_char, one, two);

            *acc.entry(one).or_default() += value;  // For the next cycle.
            *acc.entry(two).or_default() += value;  // For the next cycle.
            *count_chars.entry(new_char).or_default() += value;  // Increment the count!

            acc
        });
        // display_sorted(&count_pairs);
        // display_sorted(&count_chars);
    }
    println!("FINSHED CYCLES:");
    display_sorted(&count_pairs);
    display_sorted(&count_chars);

    let just_nums: Vec<i64> = count_chars.values().cloned().collect();  // Might be better to just use lazy eval, rather than cloned()?
    let char_min = just_nums.iter().min().unwrap();
    let char_max = just_nums.iter().max().unwrap();
    println!("SCORE: {}", *char_max - *char_min);
}




    // // Do one cycle -- go over every letter. Assemble a new array with the new letters.
    // // fn cycle(in_starting: &Vec<char>, pairs: &HashMap<&str, &str>) -> Vec<char> {
    // //     let mut ending: Vec<char> = vec![];
    // //     // let mut counts: HashMap<String, i32> = HashMap::new();
    // //     for idx in 0..(in_starting.len() - 1) {
    // //         let pair: String = format!("{}{}", in_starting[idx], in_starting[idx + 1]);
    // //         match pairs.get(&pair as &str) {
    // //             Some(x) => {
    // //                     ending.push(in_starting[idx]);
    // //                     ending.push(x.chars().next().unwrap());
    // //                 },
    // //             None => (),
    // //         }
    // //     }
    // //     ending.push(*in_starting.iter().last().unwrap());

    // //     return ending;
    // // }

    // // Do one cycle -- go over every letter. Assemble a new array with the new letters.
    // fn cycle(in_starting: &mut String, pairs: &HashMap<&str, &str>) {
    //     let orig_starting = in_starting.clone();
    //     let mut new_string = "".to_string();
    //     for idx in 0..(orig_starting.len() - 1) {
    //         let pair = &orig_starting[idx..(idx + 2)];
    //         let pair_0 = &orig_starting[idx..(idx + 1)];
    //         match pairs.get(&pair as &str) {
    //             Some(x) => {
    //                     new_string.push_str(pair_0);
    //                     new_string.push_str(*x);
    //                 },
    //             None => (),
    //         }
    //     }
    //     new_string.push_str(&orig_starting[orig_starting.len() - 1..]);
    //     *in_starting = new_string;
    // }

    // // Grow each pair through the entire cycle to create a lookup table of frequencies.
    // const NUM_CYCLES: i32 = 5;
    // fn grow_and_count(in_letters: &str, pairs: &HashMap<&str, &str>) -> HashMap<char, i32> {
    //     let mut new_letters = in_letters.to_string();
    //     for _num in 1..=NUM_CYCLES {
    //         cycle(&mut new_letters, &pairs);
    //         // println!("{}: {}", _num, new_letters)
    //         // println!("{}: {}", in_letters, _num)
    //     }

    //     new_letters[0..(new_letters.len() - 1)].chars().fold(HashMap::new(), |mut acc, this_char| {
    //         match acc.get_mut(&this_char) {
    //             Some(x) => *x += 1,
    //             None => { acc.insert(this_char, 1); },
    //         }
    //         acc
    //     })
    // }

    // // For each pair, cycle them the requisite number of times and note the frequency of each letter.
    // fn create_lookups(pairs: &HashMap<&str, &str>) -> HashMap<String, HashMap<char, i32>> {
    //     let mut lookups: HashMap<String, HashMap<char, i32>> = HashMap::new();
    //     for this_pair in pairs.keys() {
    //         let freq_table: HashMap<char, i32> = grow_and_count(&this_pair, &pairs);
    //         lookups.insert(this_pair.to_string(), freq_table);
    //     }

    //     lookups
    // }


    // let lookups: HashMap<String, HashMap<char, i32>> = create_lookups(&pairs);
    // //Display.
    // // for (k, v) in lookups.iter() { println!("{}: {:?}", k, v); }
    // let mut lookup_keys: Vec<&String> = lookups.keys().collect();
    // lookup_keys.sort();
    // lookup_keys.iter().for_each(|k| {
    //     print!("{}: ", k);
    //     let v: &HashMap<char, i32> = lookups.get(*k).unwrap();
    //     let mut v_lookup_keys: Vec<&char> = v.keys().collect();
    //     v_lookup_keys.sort();
    //     v_lookup_keys.iter().for_each(|vk| print!("{}: {}, ", vk, v.get(vk).unwrap()));
    //     println!();
    // });

    // let mut counts: HashMap<char, i32> = HashMap::new();
    // for x in 0..(starting.len() - 1) {
    //     let pair: String = starting[x..x+2].iter().collect();
    //     // println!("{}", pair);
    //     let pair_counts: &HashMap<char, i32> = lookups.get(&pair).unwrap();

    //     if counts.is_empty() {
    //         counts = pair_counts.clone();
    //     }
    //     else {
    //         for (k, v) in pair_counts.iter() {
    //             match counts.get_mut(&k) {
    //                 Some(x) => *x += v,
    //                 None => { counts.insert(*k, *v); }
    //             }
    //         }
    //     }
    // }
    // // The last letter needs to be appended, since while avoiding double counts on lookup borders, it doesn't get counted.
    // if let Some(x) = counts.get_mut(&starting[(starting.len() - 1)]) { *x += 1 }
    // // println!("{:?}", counts);
    // let mut count_keys: Vec<&char> = counts.keys().collect();
    // count_keys.sort();
    // // println!("{:?}", count_keys);
    // count_keys.iter().for_each(|k| print!("{}: {}, ", k, counts.get(k).unwrap()));

    // let mut all_scores: Vec<i32> = counts.values().cloned().collect();
    // all_scores.sort();
    // // println!("{:?}", all_scores);
    // println!("SCORE {:?}", all_scores[all_scores.len() - 1] - all_scores[0]); 





    // // Using percent chance of appearance.
    // // let num_cycles = 40;

    // // let pair_values: Vec<&str> = pairs.values().cloned().collect();
    // // let num_b = pair_values.iter().fold(0, | mut acc, x | { if x == &"B" { acc += 1; } acc });
    // // let per_b = num_b as f64 / pairs.len() as f64;
    // // let num_h = pair_values.iter().fold(0, | mut acc, x | { if x == &"H" { acc += 1; } acc });
    // // let per_h = num_h as f64 / pairs.len() as f64;
    // // let num_c = pair_values.iter().fold(0, | mut acc, x | { if x == &"C" { acc += 1; } acc });
    // // let per_c = num_c as f64 / pairs.len() as f64;
    // // let num_n = pair_values.iter().fold(0, | mut acc, x | { if x == &"N" { acc += 1; } acc });
    // // let per_n = num_n as f64 / pairs.len() as f64;
    // // // println!("P_V: {:?} {} {}", pair_values, num_c, num_h);
    // // println!("PERC: {} {} {} {}", num_b, per_b, num_h, per_h);

    // // let mut length: i64 = 4;
    // // println!("[0] L: {}", length);
    // // for idx in 1..(num_cycles + 1) {  // Starting with 1 for the debug output.
    // //     length += length - 1;
    // //     println!("[{}] L: {} B: {} H: {} C:{} N:{}", idx, length, per_b * (length as f64), per_h * (length as f64), per_c * (length as f64), per_n * (length as f64));
    // // }


    // // let mut ending: Vec<char> = vec![];

    // let num_cycles = 40;
    // let mut working = starting.iter().collect();
    // // println!("[{}] {}", 0, working.iter().collect::<String>());
    // for idx in 1..(num_cycles + 1) {  // Starting with 1 for the debug output.
    //     // working = cycle(&mut working, &pairs);
    //     cycle(&mut working, &pairs);
    //     // println!("[{}] {:?}", idx, working);
    //     println!("[{}]", idx);

    //     // let counts: HashMap<char, i32> = working.chars().fold(HashMap::new(), | mut acc, this_char | {
    //     //     match acc.get_mut(&this_char) {
    //     //         Some(x) => *x += 1,
    //     //         None => { acc.insert(this_char, 1); },
    //     //     }
    //     //     acc
    //     // });
    //     // let count_b = counts.get(&'B').unwrap();
    //     // let count_h = counts.get(&'H').unwrap();
    //     // let count_c = counts.get(&'C').unwrap();
    //     // let count_n = counts.get(&'N').unwrap();
    //     // let working_size = working.len() as f64;
    //     // println!("B: {} {} H: {} {} C: {} {} N: {} {}",
    //     //     count_b, *count_b as f64 / working_size,
    //     //     count_h, *count_h as f64 / working_size,
    //     //     count_c, *count_c as f64 / working_size,
    //     //     count_n, *count_n as f64 / working_size
    //     // );

    //     // let mut all_scores: Vec<i32> = counts.values().cloned().collect();
    //     //all_scores.sort();
    //     // println!("{:?}", all_scores);
    // }

    // let counts: HashMap<char, i32> = working.chars().fold(HashMap::new(), | mut acc, this_char | {
    //     match acc.get_mut(&this_char) {
    //         Some(x) => *x += 1,
    //         None => { acc.insert(this_char, 1); },
    //     }
    //     acc
    // });
    // println!("{:?}", counts);
    // let mut all_scores: Vec<i32> = counts.values().cloned().collect();
    // all_scores.sort();
    // // println!("{:?}", all_scores);
    // println!("SCORE {:?}", all_scores[all_scores.len() - 1] - all_scores[0]);
