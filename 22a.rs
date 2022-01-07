pub mod usrlib;

fn main() {
    let input_stuff = [
        // "on x=10..12,y=10..12,z=10..12",
        // "on x=11..13,y=11..13,z=11..13",
        // "off x=9..11,y=9..11,z=9..11",
        // "on x=10..10,y=10..10,z=10..10",

        "on x=-20..26,y=-36..17,z=-47..7",
        "on x=-20..33,y=-21..23,z=-26..28",
        "on x=-22..28,y=-29..23,z=-38..16",
        "on x=-46..7,y=-6..46,z=-50..-1",
        "on x=-49..1,y=-3..46,z=-24..28",
        "on x=2..47,y=-22..22,z=-23..27",
        "on x=-27..23,y=-28..26,z=-21..29",
        "on x=-39..5,y=-6..47,z=-3..44",
        "on x=-30..21,y=-8..43,z=-13..34",
        "on x=-22..26,y=-27..20,z=-29..19",
        "off x=-48..-32,y=26..41,z=-47..-37",
        "on x=-12..35,y=6..50,z=-50..-2",
        "off x=-48..-32,y=-32..-16,z=-15..-5",
        "on x=-18..26,y=-33..15,z=-7..46",
        "off x=-40..-22,y=-38..-28,z=23..41",
        "on x=-16..35,y=-41..10,z=-47..6",
        "off x=-32..-23,y=11..30,z=-14..3",
        "on x=-49..-5,y=-3..45,z=-29..18",
        "off x=18..30,y=-20..-8,z=-3..13",
        "on x=-41..9,y=-7..43,z=-33..15",
        // "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
        // "on x=967..23432,y=45373..81175,z=27513..53682",
    ];
    // let input_stuff = usrlib::vec_lines_from_file("22.in.txt");

    let mut core: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 101]; 101]; 101];  // The core, intialized to 0s

    'outer: for line_num in 0..input_stuff.len() {
        let line = &input_stuff[line_num];
        let tokens: Vec<&str> = line.split(' ').collect();


        // Split up each line into tokens and extract the range numbers.
        let xyzs: Vec<&str> = tokens[1].split(',').collect();
        let mut axis_and_ranges: Vec<Vec<i32>> = vec![];
        for idx in 0..xyzs.len() {
            let coords = xyzs[idx];
            let coords_tokens: Vec<&str> = coords.split("=").collect();
            let mut ranges: Vec<i32> = coords_tokens[1].split("..").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
            ranges.sort();  // Lower always first.
            // println!("RANGES: {:?}", ranges);
            if ranges[0] >= -50 && ranges[0] <= 50 && ranges[1] >= -50 && ranges[1] <= 50 {
                axis_and_ranges.push((ranges[0]..=ranges[1]).collect::<Vec<i32>>());
            }
            else {
                continue 'outer;
            }
        }

        if tokens[0] == "on" {  // If on, turn the point into a 1.
            for x in &axis_and_ranges[0] {
                for y in &axis_and_ranges[1] {
                    for z in &axis_and_ranges[2] {
                        // core[*x as usize][*y as usize][*z as usize] = 1;
                        core[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = 1;
                    }
                }
            }
        }
        else {  // If off, turn the point into a 0.
            for x in &axis_and_ranges[0] {
                for y in &axis_and_ranges[1] {
                    for z in &axis_and_ranges[2] {
                        // core[*x as usize][*y as usize][*z as usize] = 0;
                        core[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = 0;
                    }
                }
            }
        }
    }

    // Count the number of points that have 1s
    let mut num_on = 0;
    for x in 0..core.len() {
        for y in 0..core[x].len() {
            for z in 0..core[x][y].len() {
                if core[x][y][z] == 1 {
                    num_on += 1;
                    // println!("{} {} {}", x, y, z);
                }
            }
        }
    }

    println!("NUM ON {}", num_on);
}


    // Can't skip huge numbers with map.
    // let axis_and_ranges: Vec<Vec<i32>> = tokens[1].split(',').map(|coords| {
    //     let coords_tokens: Vec<&str> = coords.split("=").collect();
    //     // let axis = coords_tokens[0];
    //     let mut ranges: Vec<i32> = coords_tokens[1].split("..").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
    //     ranges.sort();  // Lower always first.
    //     (ranges[0]..=ranges[1]).collect::<Vec<i32>>()
    // }).collect();


    // Oops. Cubes are different sizes, which means you can't track x y z independently.
    // let mut core: Vec<HashSet<i32>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
    // input_stuff.iter().for_each(|line| {
    //     let tokens: Vec<&str> = line.split(' ').collect();
    //     tokens[1].split(',').for_each(|coords| {
    //         let coords_tokens: Vec<&str> = coords.split("=").collect();
    //         let mut ranges: Vec<i32> = coords_tokens[1].split("..").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
    //         ranges.sort();  // Lower always first.

    //         if tokens[0] == "on" {
    //             for num in ranges[0]..=ranges[1] {
    //                 if coords_tokens[0] == "x" {
    //                     core[0].insert(num);
    //                 }
    //                 else if coords_tokens[0] == "y" {
    //                     core[1].insert(num);
    //                 }
    //                 else if coords_tokens[0] == "z" {
    //                     core[2].insert(num);
    //                 }
    //             }
    //         }
    //         else {
    //             for num in ranges[0]..=ranges[1] {
    //                 if coords_tokens[0] == "x" {
    //                     core[0].remove(&num);
    //                 }
    //                 else if coords_tokens[0] == "y" {
    //                     core[1].remove(&num);
    //                 }
    //                 else if coords_tokens[0] == "z" {
    //                     core[2].remove(&num);
    //                 }
    //             }
    //         }
    //     });
    // });