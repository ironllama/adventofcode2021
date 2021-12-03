pub mod usrlib;  // Yay, abstraction.

fn main() {
    let input_stuff = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    // let input_stuff = usrlib::vec_lines_from_file("3.in.txt");

    fn process(input_stuff: Vec<Vec<char>>) -> Vec<char>{
        let results = input_stuff.iter().fold(vec![0; input_stuff[0].len()], | mut acc, line | {  // Storing num of 1s in a vector.
            for x in 0..line.len() {
                if line[x] == '1' {
                    acc[x] += 1;
                }
            }
            acc  // Don't forget to return the accumulator!
        });

        let majority = (input_stuff.len() as f32 / 2 as f32).ceil() as i32;
        let gammas_arr: Vec<char> = results.iter().map(|x| if x >= &majority {'1'} else {'0'}).collect();

        return gammas_arr;
    }

    let input_stuff_vec: Vec<Vec<char>> = input_stuff.iter().map(|x| x.chars().collect()).collect();

    let gammas_vec = process(input_stuff_vec);
    // println!("V: {:?}", gammas_vec);

    let new_stuff_vec: Vec<Vec<char>> = input_stuff_vec.into_iter().filter(|line| line[0] == gammas_vec[0]).collect::<Vec<Vec<char>>>();
    println!("V: {:?}", new_stuff_vec);

    // let gammas_str = gammas_vec.iter().collect::<String>();
    // println!("V: {:?}", gammas_str);

    // let epsilon_str = gammas_vec.iter().map(|x| if *x == '1' {'0'} else {'1'}).collect::<String>();

    // let input_stuff_vec: Vec<Vec<char>> = input_stuff.iter().map(|x| x.chars().collect()).collect();

    // println!("V: {:?}", input_stuff_vec);

    // gammas_arr.iter().enumerate().fold(input_stuff_vec, | mut acc, (i, x) | {
    //     if input_stuff_vec.count() == 1 { return acc; }
    //     else if input_stuff_vec

    //     let new_stuff = acc.iter().filter(|y| y[i] == x).collect();
    // })

    // println!("G: {}", gammas_int);  // Using Debug formatting for printing out a vector.
    // println!("E: {}", epsilon_int);  // Using Debug formatting for printing out a vector.
    // println!("POWER: {}", gammas_int * epsilon_int);
    // println!("RESULT: {}", result.iter().product::<i32>());
}