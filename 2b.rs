pub mod usrlib;

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

    // Accumulator is now [forward, depth, aim]
    let result = input_stuff.iter().fold(vec![0, 0, 0], | mut acc, x | {  // Updated accumulator to include one more param (aim).
        let tokens: Vec<&str>= x.split(' ').collect();
        let dist = tokens[1].parse::<i32>().unwrap();

        match tokens[0] {
            "forward" => {  // Update the forward pos section...
                acc[0] = acc[0] + dist;
                acc[1] = acc[1] + (acc[2] * dist)  // Depth is now aim * num.
            }
            "down" => acc[2] = acc[2] + dist,  // Changes aim.
            "up" => acc[2] = acc[2] - dist, // Changes aim.
            _ => (),
        }

        // println!("CURR: {:?}", acc);
        acc
    });

    println!("POS: {:?}", result);
    println!("RESULT: {}", result[0] * result[1]);
}