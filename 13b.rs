pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     "6,10",
    //     "0,14",
    //     "9,10",
    //     "0,3",
    //     "10,4",
    //     "4,11",
    //     "6,0",
    //     "6,12",
    //     "4,1",
    //     "0,13",
    //     "10,12",
    //     "3,4",
    //     "3,0",
    //     "8,4",
    //     "1,10",
    //     "2,14",
    //     "8,10",
    //     "9,0",

    //     "fold along y=7",
    //     "fold along x=5",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("13.in.txt");

    let mut input_coords: Vec<Vec<i32>> = vec![];  // Stores the coordinates in an array of coordinates [x, y].
    let mut input_folds: Vec<(&str, i32)> = vec![];  // Stores the folds direction and location as a tuple.

    // Put the data in the structures and format that I want.
    input_stuff.iter().for_each(|line| {
        if !line.is_empty() {
            let first = line.chars().nth(0).unwrap();
            if first == 'f' {
                let tokens: Vec<&str> = line.split(' ').collect();
                let key_value: Vec<&str> = tokens[2].split('=').collect();
                input_folds.push((key_value[0], key_value[1].parse::<i32>().unwrap()));
            }
            else {
                let tokens: Vec<&str> = line.split(",").collect();
                input_coords.push(vec![tokens[0].parse::<i32>().unwrap(), tokens[1].parse::<i32>().unwrap()]);
            }
        }
    });
    // println!("COORDS: {:?}", input_coords);
    // println!("FOLDS: {:?}", input_folds);

    let mut post_fold = input_coords.clone();
    input_folds.iter().for_each(|fold_tup| {
        let fold_dir = fold_tup.0;
        let fold_at = fold_tup.1;
        post_fold = post_fold.iter().fold(vec![], |mut acc, xy| {  // Use fold to fold!
            if fold_dir == "y" {  // Folding horizontally.
                let mut new_y = xy[1];
                if new_y >= fold_at {
                    new_y = fold_at - (new_y - fold_at);
                }
                let new_coord = vec![xy[0].clone(), new_y];
                if !acc.contains(&new_coord) {
                    acc.push(new_coord);
                }
            }
            else {  // Folding vertically.
                let mut new_x = xy[0];
                if new_x >= fold_at {
                    new_x = fold_at - (new_x - fold_at);
                }
                let new_coord = vec![new_x, xy[1].clone()];
                if !acc.contains(&new_coord) {
                    acc.push(new_coord);
                }
            }
            acc
        });
    });

    // println!("POST: {:?}", post_fold);
    // println!("POST LEN: {}", post_fold.len());

    // To create the output, gonna determine the boundary of the display matrix.
    let max_x = post_fold.iter().fold(0, | mut acc, coord | { if coord[0] > acc { acc = coord[0]; } acc });
    let max_y = post_fold.iter().fold(0, | mut acc, coord | { if coord[1] > acc { acc = coord[1]; } acc });
    // println!("MAXES: {}, {}", max_x, max_y);

    // Then scan the rows, columns to find out with "pixels" to light up.
    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if post_fold.contains(&vec![x, y]) {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}