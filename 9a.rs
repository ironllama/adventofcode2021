pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "2199943210",
    //     "3987894921",
    //     "9856789892",
    //     "8767896789",
    //     "9899965678",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("9.in.txt");

    let input_stuff_vec: Vec<Vec<i32>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let line_chars: Vec<char> = line.chars().collect();
        let line_ints: Vec<i32> = line_chars.iter().map(|x| x.to_string().parse::<i32>().unwrap()).collect();
        acc.push(line_ints);
        acc
    });
    // println!("INPUT: {:?}", input_stuff_vec);

    let mut lowest = vec![];
    let mut lowest_coords = vec![];
    for y in 0..input_stuff_vec.len() {
        let line = &input_stuff_vec[y];
        for x in 0..line.len() {
            let curr = line[x];
            let mut different = false;
            if y > 0 {
                let above = input_stuff_vec[y - 1][x];
                if above < curr {
                    continue;
                }
                if above != curr {
                    different = true;
                }
            }
            if x > 0 {
                let left = line[x - 1];
                if left < curr {
                    continue;
                }
                if left != curr {
                    different = true;
                }
            }
            if x < (line.len() - 1) {
                let right = line[x + 1];
                if right < curr {
                    continue;
                }
                if right != curr {
                    different = true;
                }
            }
            if y < (input_stuff_vec.len() - 1) {
                let below = input_stuff_vec[y + 1][x];
                if below < curr {
                    continue;
                }
                if below != curr {
                    different = true;
                }
            }
            if different {
                lowest.push(curr);
                lowest_coords.push(vec![x, y]);
            }
        }
    }
    println!("LOWS: {:?}", lowest);
    println!("CRDS: {:?}", lowest_coords);

    // for y in 0..input_stuff_vec.len() {
    //     let mut line = "";
    //     for x in 0..line.len() {

    //     }
    // }

    let scores: Vec<i32> = lowest.iter().map(|x| x + 1).collect();
    println!("SCORE: {}", scores.iter().sum::<i32>());
}