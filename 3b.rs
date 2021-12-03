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

    let input_stuff_vec: Vec<Vec<char>> = input_stuff.iter().map(|x| x.chars().collect()).collect();  // Easier to use when chars are separated.

    // To get which columns have 1 as a majority.
    fn process(input_stuff: &Vec<Vec<char>>) -> Vec<char> {  // Accept by ref, to avoid moving.
        let results = input_stuff.iter().fold(vec![0; input_stuff[0].len()], | mut acc, line | {  // Storing num of 1s in a vector.
            for x in 0..line.len() {
                if line[x] == '1' {
                    acc[x] += 1;
                }
            }
            acc  // Don't forget to return the accumulator!
        });

        let majority = (input_stuff.len() as f32 / 2 as f32).ceil() as i32;
        let gammas_arr: Vec<char> = results.iter().map(|x| if x == &majority {'e'} else if x > &majority {'1'} else {'0'}).collect();

        return gammas_arr;
    }

    // Convenience since we have to run both o2 and co2, but with only the difference that co2 runs with opposite bits.
    fn get_rating(sensor: &str, input_stuff_vec: &Vec<Vec<char>>) -> isize {
        let mut idx = 0;
        let mut rating_vec = input_stuff_vec.clone();
        while rating_vec.len() > 1 {
            let mut next_vec = process(&rating_vec);
            if sensor == "co2" {  // Flip the bits!
                next_vec = next_vec.iter().map(|x| if *x == 'e' {'e'} else if *x == '1' {'0'} else {'1'}).collect();
            }
            // println!("NV: {:?}", new_next_vec);
            rating_vec = rating_vec.into_iter()
                .filter(|line|
                    if next_vec[idx] == 'e' {
                        if sensor == "co2" { return line[idx] == '0' }  // Different default values when equal.
                        else { return line[idx] == '1'}
                    }
                    else {
                        return line[idx] == next_vec[idx]
                    })
                .collect();
            // println!("ISV: {:?}", rating_vec);
            idx += 1;
        }
        // println!("CO2: {:?}", rating_vec);
        let rating_str = rating_vec[0].iter().collect::<String>();
        let rating_int = isize::from_str_radix(&rating_str, 2).unwrap();

        return rating_int;
    }

    let o2_int = get_rating("o2", &input_stuff_vec);
    let co2_int = get_rating("co2", &input_stuff_vec);
    println!("LIFE SUPPORT RATING: {}", o2_int * co2_int);
}