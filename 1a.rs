use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    // str::FromStr,
    // convert::TryInto,
};

// fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
//     v.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

// Returns a Vector
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    // let input_stuff = [
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

    // let input_stuff: Vec<i32> = lines_from_file("1.in.txt").unwrap().iter().map(|x| x.parse::<i32>()).collect().unwrap();  // Wat.
    // println!("{:?}", input_stuff);

    // So much confusion. How does this all work?
    // let input_stuff = lines_from_file("1.in.txt").unwrap();
    // let input_stuff = match lines_from_file("1.in.txt").map(|x| x.parse::<u16>().unwrap()) {
    // let input_stuff = match lines_from_file("1.in.txt").map(|x| i16::from_str(x).unwrap()) {
    //     Ok(input_stuff) => input_stuff,
    //     Err(e) => {
    //         println!("error: {}", e);
    //         vec![]
    //     }
    // };

    let mut increasing = 0;

    for num in 1..(input_stuff.len()) {
        if input_stuff[num] > input_stuff[num - 1] {
            // println!("{} (increased)", input_stuff[num]);
            increasing = increasing + 1;
        }
        else {
            // println!("{} (decreased)", input_stuff[num]);
        }
    }

    println!("NUM: {}", increasing);
}