pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "16,1,2,0,4,2,7,1,2,14",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("7.in.txt");

    let input_stuff_vec: Vec<i32> = input_stuff[0]
                                        .split(',')
                                        .collect::<Vec<&str>>()
                                        .iter()
                                        .map(|x| x.parse::<i32>().unwrap())
                                        .collect();

    let min_num = *input_stuff_vec.iter().min().unwrap();
    let max_num = *input_stuff_vec.iter().max().unwrap();

    let mut least_fuel = 0;
    let mut least_fuel_pos = -1;

    // Test every possible position from end to end.
    for pos in min_num..(max_num + 1) {
        let mut pos_fuel_used = 0;

        for num in &input_stuff_vec {
            // let fuel_used: i32 = (0..(num_pos + 1)).fold(0, | mut acc, x | { acc += x; acc } );  // Yikes, slow.
            let dist = (num - pos).abs();
            let fuel_used: i32 = (dist as f32 * ((dist + 1).abs() as f32 / 2.0)) as i32;

            pos_fuel_used += fuel_used;
            // println!("Move from {} to {}: {} fuel", num, pos, fuel_used);
        }

        if least_fuel == 0 || pos_fuel_used < least_fuel {
            least_fuel = pos_fuel_used;
            least_fuel_pos = pos;
        }
    }

    println!("LEAST FUEL: {} {}", least_fuel_pos, least_fuel);
}