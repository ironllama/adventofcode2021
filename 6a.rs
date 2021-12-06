pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "3,4,3,1,2"
    // ];
    let input_stuff = usrlib::vec_lines_from_file("6.in.txt");

    let input_vec: Vec<&str> = input_stuff[0].split(',').collect();
    let input_vec_ints: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    fn next_day(all_fish: Vec<i32>) -> Vec<i32> {
        let mut new_all_fish = all_fish;
        for idx in 0..new_all_fish.len() {
            if new_all_fish[idx] == 0 {
                new_all_fish[idx] = 6;
                new_all_fish.push(8);
            }
            else {
                new_all_fish[idx] -= 1;
            }
            // println!("F: {}", idx);
        }
        return new_all_fish;
    }

    let mut result = input_vec_ints;
    // for cycles in 1..18 + 1 {
    for cycles in 1..80 + 1 {
        result = next_day(result);
        // println!("CYCLE {} -> {:?}", cycles, result);
    }

    // println!("FINAL {} {:?}", result.len(), result);
    println!("FINAL {}", result.len());
}