use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// Read file.
pub fn result_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// Read file into a Vector.
pub fn vec_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    result_lines_from_file(filename).unwrap()
}

// Convert a vector of string into int.
pub fn vec_str_to_int(input: Vec<String>) -> Vec<i32> {
    input
    .iter()
    .map(|x| x.parse::<i32>().unwrap())
    .collect()
}

pub fn swap_nums(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}