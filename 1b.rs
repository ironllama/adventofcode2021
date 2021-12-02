use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// Returns a Vector
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    // let input_stuff = vec![
    //     199,
    //     200,
    //     208,
    //     210,
    //     200,
    //     207,
    //     240,
    //     269,
    //     260,
    //     263,
    // ];

    let input_strings = lines_from_file("1.in.txt").unwrap();
    let input_stuff: Vec<i32> = input_strings
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut increasing = 0;
    let mut last_group = 0;

    for num in 1..(input_stuff.len() - 2) {  // Limit at -2 since we're going by groups of 3.
        let new_group = input_stuff[num] + input_stuff[num + 1] + input_stuff[num + 2];
        if new_group != 0 && new_group > last_group {
            // println!("{} (increased)", input_stuff[num]);
            increasing = increasing + 1;
        }
        last_group = new_group;
    }

    println!("NUM: {}", increasing);
}