pub mod usrlib;  // Yay, abstraction.

fn main() {
    // let input_stuff = vec![
    //     "0,9 -> 5,9",
    //     "8,0 -> 0,8",
    //     "9,4 -> 3,4",
    //     "2,2 -> 2,1",
    //     "7,0 -> 7,4",
    //     "6,4 -> 2,0",
    //     "0,9 -> 2,9",
    //     "3,4 -> 1,4",
    //     "0,0 -> 8,8",
    //     "5,5 -> 8,2",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("5.in.txt");

    let mut max_x = 0;
    let mut max_y = 0;

    // Get coordinates in a vector.
    let all_coords: Vec<Vec<Vec<i32>>> = input_stuff.iter().fold(vec![], | mut acc, line | {
        let start_end_str: Vec<&str> = line.split(" -> ").collect();
        // println!("{:?}", start_end_str);
        let start_end_int: Vec<Vec<i32>> = start_end_str.iter().map(|x| {
            let x_y_str: Vec<&str> = x.split(',').collect();
            let x_y_int: Vec<i32> = x_y_str.iter().map(|num| num.parse::<i32>().unwrap()).collect();
            if x_y_int[0] > max_x { max_x = x_y_int[0] }
            if x_y_int[1] > max_y { max_y = x_y_int[1] }
            x_y_int
        }).collect();
        // println!("{:?}", start_end_int);
        acc.push(start_end_int);
        acc
    });
    // println!("MAX X: {} MAX Y: {} ALL: {:?}", max_x, max_y, all_coords);

    // Prepare a field of 0's -- NOTE field is [Y][X], not [X][Y], because rows(y) are made before columns(x).
    let mut field = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    // for line in &field { println!("{:?}", line )}

    // "Draw" in all the lines and counts.
    for line in all_coords {
        if line[0][0] == line[1][0] {  // If x is the same...
            let mut start = line[0][1];
            let mut end = line[1][1];
            if start > end {
                start = start ^ end;
                end = start ^ end;
                start = start ^ end;
            }
            for x in start..end + 1 {  // Draw along Y (vertical)
                field[x as usize][line[0][0] as usize] += 1;
            }
        }
        else if line[0][1] == line[1][1] {  // If the y is the same...
            let mut start = line[0][0];
            let mut end = line[1][0];
            if start > end {
                start = start ^ end;
                end = start ^ end;
                start = start ^ end;
            }
            for y in start..end + 1 {  // Draw along X (horizontal)
                field[line[0][1] as usize][y as usize] += 1;
            }
        }
    }

    // println!("MAP: ");
    // for line in &field { println!("{:?}", line )}

    // Count overlapping points.
    let num_overlap: i32 = field.into_iter().fold(0, |mut  acc, line | {
        for pt in line { if pt > 1 { acc += 1 }}
        acc
    });

    println!("OVERLAPS: {}", num_overlap);
}