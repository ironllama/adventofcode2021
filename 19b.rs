pub mod usrlib;

use std::collections::VecDeque;

fn main() {
    let input_stuff = usrlib::vec_lines_from_file("19.in.txt");

    // Initialize input into a data structure of numbers.
    let mut input_stuff_vec: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut scanner_vec = vec![];
    for idx in 0..input_stuff.len() {
        if input_stuff[idx].is_empty() {  // Blank lines between scanners.
            input_stuff_vec.push(scanner_vec);
            scanner_vec = vec![];
        }
        else if input_stuff[idx].chars().collect::<Vec<char>>()[1] != '-' {  // Need to get second -, since negative numbers also start with -
            let new_coords_vec: Vec<i32> = input_stuff[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            let new_coords: (i32, i32, i32) = (new_coords_vec[0], new_coords_vec[1], new_coords_vec[2]);
            scanner_vec.push(new_coords);
        }
    }
    input_stuff_vec.push(scanner_vec);  // Don't forget the last set!
    // println!("INPUT: {:?}", input_stuff_vec);

    // Compare the beacon list of two scanners (pre-oriented) by translating their location in space (offset).
    fn compare_beacons(one_vec: &Vec<(i32, i32, i32)>, two_vec: &mut Vec<(i32, i32, i32)>, offset: &mut (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let mut scanner_matches: Vec<(i32, i32, i32)> = vec![];

        // Comparing the two scanners' beacon lists. Every beacon from the first list is a possible "origin" beacon, since not all beacons will overlap.
        let mut test_vec: VecDeque<(i32, i32, i32)> = VecDeque::from(two_vec.clone());  // Since we manipulate, create a separate copy.
        'outer: for uno in 0..one_vec.len() {
            for _dos in 0..test_vec.len() {  // Try all the beacons as the "origin" beacon.
                let uno_vertex: (i32, i32, i32) = one_vec[uno];
                scanner_matches = vec![];

                let dos_vertex: (i32, i32, i32) = test_vec[0];
                *offset = (dos_vertex.0 - uno_vertex.0, dos_vertex.1 - uno_vertex.1, dos_vertex.2 - uno_vertex.2);
                // println!("\tOFFSET: {:?}", offset);

                test_vec.iter().for_each(|xyz| {  // Update the values of test_vec to the new orientation.
                    let new_xyz = (xyz.0 - offset.0, xyz.1 - offset.1, xyz.2 - offset.2);
                    if one_vec.contains(&new_xyz) {
                        // println!("MATCHES! {:?}", new_xyz);
                        scanner_matches.push(new_xyz);
                    }
                });

                if scanner_matches.len() >= 12 { // Found a pair and orientation that matches!);
                    // println!("COMPARE MATCHED! {:?}", offset);
                    test_vec.iter_mut().for_each(|xyz| {  // Change all the points in second beacon list to be from the perspective of scanner one.
                        *xyz = (xyz.0 - offset.0, xyz.1 - offset.1, xyz.2 - offset.2);
                    });
                    *two_vec = Vec::from(test_vec);
                    break 'outer;
                }

                test_vec.rotate_left(1);  // Move vertices over by 1 -- try a different point as "origin".
            }
        }

        scanner_matches
    }

    // Right to up -- about the Z -- Around X, 90deg -- Y to the right
    fn clockwise(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.1); tup.1 *= -1; }); }
    fn flip_clock(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.1 *= -1; }); }
    fn anti_clock(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.1); tup.0 *= -1; }); }

    // Right to back -- about the Y -- Around Y, 90deg -- X to the back
    fn spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.2); tup.2 *= -1; }); }
    fn flip_spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.2 *= -1 }); }
    fn anti_spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.2); tup.0 *= -1 }); }

    // Front to up -- about the X -- Around Y, 90deg -- Y to the back
    fn pitch(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.1, &mut tup.2); tup.2 *= -1; }); }
    fn anti_pitch(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.1, &mut tup.2); tup.1 *= -1; }); }

    // Convenient way to rotate the beacon list to each of the 24 possible orientations.
    fn rotate_beacons(beacon_list: &mut Vec<(i32, i32, i32)>, orientation: i32) {
        // let new_list: Vec<(i32, i32, i32)> = beacon_list;  // Do we even need to clone? Might be able to just edit original.
        match orientation {
            0 => (),  // Original
            1 => clockwise(beacon_list),
            2 => flip_clock(beacon_list),
            3 => anti_clock(beacon_list),

            4 => { spin(beacon_list) }
            5 => { spin(beacon_list); clockwise(beacon_list) }
            6 => { spin(beacon_list); flip_clock(beacon_list) }
            7 => { spin(beacon_list); anti_clock(beacon_list) }

            8 => { flip_spin(beacon_list) }
            9 => { flip_spin(beacon_list); clockwise(beacon_list) }
            10 => { flip_spin(beacon_list); flip_clock(beacon_list) }
            11 => { flip_spin(beacon_list); anti_clock(beacon_list) }

            12 => { anti_spin(beacon_list) }
            13 => { anti_spin(beacon_list); clockwise(beacon_list) }
            14 => { anti_spin(beacon_list); flip_clock(beacon_list) }
            15 => { anti_spin(beacon_list); anti_clock(beacon_list) }

            16 => { pitch(beacon_list) }
            17 => { pitch(beacon_list); clockwise(beacon_list) }
            18 => { pitch(beacon_list); flip_clock(beacon_list) }
            19 => { pitch(beacon_list); anti_clock(beacon_list) }

            20 => { anti_pitch(beacon_list) }
            21 => { anti_pitch(beacon_list); clockwise(beacon_list) }
            22 => { anti_pitch(beacon_list); flip_clock(beacon_list) }
            23 => { anti_pitch(beacon_list); anti_clock(beacon_list) }

            _ => (),
        };
    }

    // Try to figure out if two scanner lists are overlapping.
    fn process_two(one: usize, two: usize, input_stuff_vec: &mut Vec<Vec<(i32, i32, i32)>>, new_beacons: &mut Vec<(i32, i32, i32)>, scanner_locations: &mut Vec<(i32, i32, i32, i32)>) -> usize {
        let mut return_val: usize = 0;

        for or_idx in 0..24 {  // 24 orientations of the beacon list.
            let mut one_vec = input_stuff_vec[one].clone();
            let mut two_vec: Vec<(i32, i32, i32)> = input_stuff_vec[two].clone();  // Manipulate a fresh copy. Rotation starts from same point each time.

            if scanner_locations[one] == (0, 0, 0, 0) && scanner_locations[two] != (0, 0, 0, 0) {
                let temp_vec = one_vec;
                one_vec = two_vec;
                two_vec = temp_vec;
            }

            rotate_beacons(&mut two_vec, or_idx);

            let mut scanner_two_offset = (0, 0, 0);
            let new_matches: Vec<(i32, i32, i32)> = compare_beacons(&one_vec, &mut two_vec, &mut scanner_two_offset);

            if new_matches.len() >= 12 {
                    scanner_locations[two] = (scanner_two_offset.0 * -1, scanner_two_offset.1 * -1, scanner_two_offset.2 * -1, or_idx);  // Location direction is opposite of offset.
                    println!("MATCHES! SCANNER {} AND {} OFFSET {:?} OR_IDX {} ADJ: {:?}", one, two, scanner_two_offset, or_idx, scanner_locations[two]);

                    input_stuff_vec[two] = two_vec.clone();  // Overwrite with orientation and points per scanner 0.

                    let not_matched: Vec<(i32, i32, i32)> = input_stuff_vec[two].iter().filter(|x| !new_matches.contains(x)).cloned().collect();
                    not_matched.iter().for_each(|x| if !new_beacons.contains(x) { new_beacons.push(*x); });

                    return_val = two;
                    break;
            }
        }

        return_val
    }

    let mut new_beacons: Vec<(i32, i32, i32)> = input_stuff_vec[0].clone();  // Pre-populate with beacons from scanner 0.
    let mut scanner_locations: Vec<(i32, i32, i32, i32)> = vec![(0, 0, 0, 0); input_stuff_vec.len()];  // x, y, z, orientation

    // Better to just analyze each scanner as their location from scanner 0's perspective is discovered.
    let mut todo: Vec<usize> = vec![0];
    while !todo.is_empty() {
        let one: usize = todo.pop().unwrap();
        for two in 0..input_stuff_vec.len() {
            if one != two && two != 0 && scanner_locations[two] == (0, 0, 0, 0) {
                let ret_val = process_two(one, two, &mut input_stuff_vec, &mut new_beacons, &mut scanner_locations);
                if ret_val != 0 {
                    todo.push(ret_val);
                }
            }
        }
    }


    println!("OFFSETS");
    scanner_locations.iter().for_each(|x| println!("{:?}", x));
    println!();
    // println!("BEACONS:");
    // new_beacons.sort();
    // new_beacons.iter().for_each(|x| println!("{:?}", x));
    // println!("NUM {}", new_beacons.len());

    let mut largest_distance = 0;
    let mut largest_distance_pair = vec![];
    for one in 0..scanner_locations.len() {
        for two in (one + 1)..scanner_locations.len() {
            let this_distance = (scanner_locations[one].0 - scanner_locations[two].0).abs()
                + (scanner_locations[one].1 - scanner_locations[two].1).abs()
                + (scanner_locations[one].2 - scanner_locations[two].2).abs();
            
            if this_distance > largest_distance {
                largest_distance = this_distance;
                largest_distance_pair = vec![scanner_locations[one], scanner_locations[two]];
            }
        }
    }

    println!("PAIR: {:?}", largest_distance_pair);
    println!("DIST: {}", largest_distance);
}


// To save time, you can also run it with the resulting scanner locations from part a.
// let input_stuff: Vec<(i32, i32, i32, i32)> = vec![
//     // (0, 0, 0, 0),
//     // (68, -1246, -43, 8),
//     // (1105, -1205, 1229, 22),
//     // (-92, -2380, -20, 8),
//     // (-20, -1133, 1061, 15),
//     (0, 0, 0, 0),
//     (1177, -3629, -3598, 3),
//     (31, -1141, -108, 12),
//     (21, -2481, 1192, 15),
//     (-1282, -4724, -1244, 4),
//     (86, 2435, 1194, 16),
//     (-1219, -1170, -132, 21),
//     (1147, -2311, 1177, 7),
//     (47, -2308, 2316, 18),
//     (1168, -4707, -3715, 23),
//     (1230, -3618, -1294, 13),
//     (-28, 34, -1193, 20),
//     (-1259, 2498, -20, 11),
//     (-55, -3644, 1068, 17),
//     (-84, -4881, -1242, 6),
//     (42, 2425, -65, 2),
//     (1252, -2398, -73, 5),
//     (-72, -2434, -35, 1),
//     (-80, -2352, -1205, 8),
//     (85, 1174, 1136, 19),
//     (1248, -1258, 1039, 14),
//     (1107, 35, -114, 22),
//     (-1293, 2463, -1299, 9),
//     (-40, 1258, -45, 10),
//     (-54, -4749, -2526, 10),
//     (1197, -3616, -2396, 8),
//     (1209, -4813, -1358, 1),
//     (-93, -3519, -1258, 9),
//     (-1267, -36, -1321, 8),
//     (1218, -1124, 2412, 9),
// ];