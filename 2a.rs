pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "forward 5",
    //     "down 5",
    //     "forward 8",
    //     "up 3",
    //     "down 8",
    //     "forward 2",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("2.in.txt");

    let result = input_stuff.iter().fold(vec![0, 0], | mut acc, x | {  // Storing results in a [forward, depth] vector.
        let tokens: Vec<&str>= x.split(' ').collect();  // Splitting up each line by space into a vector.
        let dist = tokens[1].parse::<i32>().unwrap();  // Casting to number.

        match tokens[0] {  // Fancy switch/case...
            "forward" => acc[0] = acc[0] + dist,
            "down" => acc[1] = acc[1] + dist,
            "up" => acc[1] = acc[1] - dist,
            _ => (),  // Required catch-all.
        }

        acc  // Don't forget to return the accumulator!
    });

    println!("POS: {:?}", result);  // Using Debug formatting for printing out a vector.
    println!("RESULT: {}", result.iter().product::<i32>());
}