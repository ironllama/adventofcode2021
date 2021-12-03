pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "00100",
    //     "11110",
    //     "10110",
    //     "10111",
    //     "10101",
    //     "01111",
    //     "00111",
    //     "11100",
    //     "10000",
    //     "11001",
    //     "00010",
    //     "01010",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("3.in.txt");

    let results = input_stuff.iter().fold(vec![0; input_stuff[0].len()], | mut acc, x | {  // Storing num of 1s in a vector.
        let tokens: Vec<char>= x.chars().collect();  // Splitting up each char into a vector.
        // println!("ACC: {:?}", acc);

        for x in 0..tokens.len() {
            if tokens[x] == '1' {
                acc[x] += 1;
            }
        }

        acc  // Don't forget to return the accumulator!
    });
    // println!("RESULTS: {:?}", results);  // Using Debug formatting for printing out a vector.

    let majority = (input_stuff.len() as f32 / 2 as f32).ceil() as i32;

    let gammas_arr: Vec<char> = results.iter().map(|x| if x > &majority {'1'} else {'0'}).collect();
    let gammas_str = gammas_arr.iter().collect::<String>();
    let gammas_int = isize::from_str_radix(&gammas_str, 2).unwrap();

    let epsilon_str = gammas_arr.iter().map(|x| if *x == '1' {'0'} else {'1'}).collect::<String>();
    let epsilon_int = isize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("G: {}", gammas_int);  // Using Debug formatting for printing out a vector.
    println!("E: {}", epsilon_int);  // Using Debug formatting for printing out a vector.
    println!("POWER: {}", gammas_int * epsilon_int);
    // println!("RESULT: {}", result.iter().product::<i32>());
}