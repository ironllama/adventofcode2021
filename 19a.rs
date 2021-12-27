pub mod usrlib;

use std::collections::VecDeque;

fn main() {
    // let input_stuff = [
    //     // "--- scanner 0 ---",
    //     // "-1,-1,1",
    //     // "-2,-2,2",
    //     // "-3,-3,3",
    //     // "-2,-3,1",
    //     // "5,6,-4",
    //     // "8,0,7",
    //     // "",
    //     // "--- scanner 1 ---",
    //     // "1,-1,1",
    //     // "2,-2,2",
    //     // "3,-3,3",
    //     // "2,-1,3",
    //     // "-5,4,-6",
    //     // "-8,-7,0",
    //     // "",
    //     // "--- scanner 2 ---",
    //     // "-1,-1,-1",
    //     // "-2,-2,-2",
    //     // "-3,-3,-3",
    //     // "-1,-3,-2",
    //     // "4,6,5",
    //     // "-7,0,8",
    //     // "",
    //     // "--- scanner 3 ---",
    //     // "1,1,-1",
    //     // "2,2,-2",
    //     // "3,3,-3",
    //     // "1,3,-2",
    //     // "-4,-6,5",
    //     // "7,0,8",
    //     // "",
    //     // "--- scanner 4 ---",
    //     // "1,1,1",
    //     // "2,2,2",
    //     // "3,3,3",
    //     // "3,1,2",
    //     // "-6,-4,-5",
    //     // "0,7,-8",

    //     "--- scanner 0 ---",
    //     "404,-588,-901",
    //     "528,-643,409",
    //     "-838,591,734",
    //     "390,-675,-793",
    //     "-537,-823,-458",
    //     "-485,-357,347",
    //     "-345,-311,381",
    //     "-661,-816,-575",
    //     "-876,649,763",
    //     "-618,-824,-621",
    //     "553,345,-567",
    //     "474,580,667",
    //     "-447,-329,318",
    //     "-584,868,-557",
    //     "544,-627,-890",
    //     "564,392,-477",
    //     "455,729,728",
    //     "-892,524,684",
    //     "-689,845,-530",
    //     "423,-701,434",
    //     "7,-33,-71",
    //     "630,319,-379",
    //     "443,580,662",
    //     "-789,900,-551",
    //     "459,-707,401",
    //     "",
    //     "--- scanner 1 ---",
    //     "686,422,578",
    //     "605,423,415",
    //     "515,917,-361",
    //     "-336,658,858",
    //     "95,138,22",
    //     "-476,619,847",
    //     "-340,-569,-846",
    //     "567,-361,727",
    //     "-460,603,-452",
    //     "669,-402,600",
    //     "729,430,532",
    //     "-500,-761,534",
    //     "-322,571,750",
    //     "-466,-666,-811",
    //     "-429,-592,574",
    //     "-355,545,-477",
    //     "703,-491,-529",
    //     "-328,-685,520",
    //     "413,935,-424",
    //     "-391,539,-444",
    //     "586,-435,557",
    //     "-364,-763,-893",
    //     "807,-499,-711",
    //     "755,-354,-619",
    //     "553,889,-390",
    //     "",
    //     "--- scanner 2 ---",
    //     "649,640,665",
    //     "682,-795,504",
    //     "-784,533,-524",
    //     "-644,584,-595",
    //     "-588,-843,648",
    //     "-30,6,44",
    //     "-674,560,763",
    //     "500,723,-460",
    //     "609,671,-379",
    //     "-555,-800,653",
    //     "-675,-892,-343",
    //     "697,-426,-610",
    //     "578,704,681",
    //     "493,664,-388",
    //     "-671,-858,530",
    //     "-667,343,800",
    //     "571,-461,-707",
    //     "-138,-166,112",
    //     "-889,563,-600",
    //     "646,-828,498",
    //     "640,759,510",
    //     "-630,509,768",
    //     "-681,-892,-333",
    //     "673,-379,-804",
    //     "-742,-814,-386",
    //     "577,-820,562",
    //     "",
    //     "--- scanner 3 ---",
    //     "-589,542,597",
    //     "605,-692,669",
    //     "-500,565,-823",
    //     "-660,373,557",
    //     "-458,-679,-417",
    //     "-488,449,543",
    //     "-626,468,-788",
    //     "338,-750,-386",
    //     "528,-832,-391",
    //     "562,-778,733",
    //     "-938,-730,414",
    //     "543,643,-506",
    //     "-524,371,-870",
    //     "407,773,750",
    //     "-104,29,83",
    //     "378,-903,-323",
    //     "-778,-728,485",
    //     "426,699,580",
    //     "-438,-605,-362",
    //     "-469,-447,-387",
    //     "509,732,623",
    //     "647,635,-688",
    //     "-868,-804,481",
    //     "614,-800,639",
    //     "595,780,-596",
    //     "",
    //     "--- scanner 4 ---",
    //     "727,592,562",
    //     "-293,-554,779",
    //     "441,611,-461",
    //     "-714,465,-776",
    //     "-743,427,-804",
    //     "-660,-479,-426",
    //     "832,-632,460",
    //     "927,-485,-438",
    //     "408,393,-506",
    //     "466,436,-512",
    //     "110,16,151",
    //     "-258,-428,682",
    //     "-393,719,612",
    //     "-211,-452,876",
    //     "808,-476,-593",
    //     "-575,615,604",
    //     "-485,667,467",
    //     "-680,325,-822",
    //     "-627,-443,-432",
    //     "872,-547,-609",
    //     "833,512,582",
    //     "807,604,487",
    //     "839,-516,451",
    //     "891,-625,532",
    //     "-652,-548,-490",
    //     "30,-46,-14",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("19.in.txt");

    let mut input_stuff_vec: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut scanner_vec = vec![];
    for idx in 0..input_stuff.len() {
        if input_stuff[idx].is_empty() {
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

    fn compare_beacons(one_vec: &Vec<(i32, i32, i32)>, two_vec: &mut Vec<(i32, i32, i32)>, offset: &mut (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let mut scanner_matches: Vec<(i32, i32, i32)> = vec![];

        // Start comparing the two scanners' beacon lists.
        let mut test_vec: VecDeque<(i32, i32, i32)> = VecDeque::from(two_vec.clone());  // Since we manipulate, create a separate copy.
        'outer: for uno in 0..one_vec.len() {
            for _dos in 0..test_vec.len() {  // Try all the beacons as the "origin" beacon.
                let uno_vertex: (i32, i32, i32) = one_vec[uno];
                scanner_matches = vec![];
                // let mut offset = (0, 0, 0);

                let dos_vertex: (i32, i32, i32) = test_vec[0];
                *offset = (dos_vertex.0 - uno_vertex.0, dos_vertex.1 - uno_vertex.1, dos_vertex.2 - uno_vertex.2);
                // *offset = (uno_vertex.0 - dos_vertex.0, uno_vertex.1 - dos_vertex.1, uno_vertex.2 - dos_vertex.2);
                // println!("\tOFFSET: {:?}", offset);
                // let offset_vertex: VecDeque<(i32, i32, i32)> = test_vec.clone();

                test_vec.iter().for_each(|xyz| {
                    let new_xyz = (xyz.0 - offset.0, xyz.1 - offset.1, xyz.2 - offset.2);
                    // let new_xyz = (xyz.0 + offset.0, xyz.1 + offset.1, xyz.2 + offset.2);
                    // if offset == (-68, 1246, 43) {
                    //     println!("\t\tCOMP: {:?} - {:?} = {:?}", xyz, offset, new_xyz);
                    // }
                    if one_vec.contains(&new_xyz) {
                        // println!("MATCHES! {:?}", new_xyz);
                        // scanner_matches.push(new_xyz);
                        scanner_matches.push(*xyz);
                    }
                });

                // if scanner_matches.len() >= 6 { // Found a pair and orientation that matches!);
                if scanner_matches.len() >= 12 { // Found a pair and orientation that matches!);
                    // println!("COMPARE MATCHED!");
                    // *two_vec = test_vec;
                    *two_vec = Vec::from(test_vec);
                    break 'outer;
                }

                test_vec.rotate_left(1);  // Move vertices over by 1 -- try a different point as "origin".
            }
        }



        // // 'outer: for _uno in 0..one_vec.len() {  // Try all the beacons as the "origin" beacon.
        // 'outer: for _dos_rotate in 0..two_vec.len() {  // Try all combinations of pairs between one and two.
        //         let mut offsets = (0, 0, 0);
        //         scanner_matches = vec![];

        //         // println!("COMPARE");
        //         for dos in 0..two_vec.len() {  // Test all the vertices as pairs -- both first, both second, both third, etc.
        //             let uno_vertex: (i32, i32, i32) = one_vec[dos];
        //             let dos_vertex: (i32, i32, i32) = two_vec[dos];
        //             // println!("\tSTARTS: {:?} {:?}", uno_vertex, dos_vertex);

        //             if dos == 0 {  // First set defines the offset used by the rest of the comparisons in the set.
        //                 // offsets = (uno_vertex.0 - dos_vertex.0, uno_vertex.1 - dos_vertex.1, uno_vertex.2 - dos_vertex.2);
        //                 // println!("\tOFFSET CALC: {:?} - {:?}", dos_vertex.0, uno_vertex.0);
        //                 offsets = (dos_vertex.0 - uno_vertex.0, dos_vertex.1 - uno_vertex.1, dos_vertex.2 - uno_vertex.2);
        //                 println!("\tOFFSET: {:?}", offsets);
        //                 // matches += 1;
        //                 scanner_matches.push(uno_vertex);
        //             }
        //             else {
        //                 // println!("\t\tCOMP: {:?} {:?} {:?}", dos_vertex.0, offsets.0, uno_vertex.0);
        //                 if offsets == (-68, 1246, 43) {
        //                     println!("\t\tCOMP: {:?} - {:?} = ({:?}) vs {:?}", dos_vertex.0, offsets.0, dos_vertex.0 - offsets.0, uno_vertex.0);
        //                 }
        //                 if (dos_vertex.0 - offsets.0) == uno_vertex.0
        //                     && (dos_vertex.1 - offsets.1) == uno_vertex.1
        //                     && (dos_vertex.2 - offsets.2) == uno_vertex.2
        //                 {
        //                     // matches += 1;
        //                     scanner_matches.push(uno_vertex);
        //                 }
        //             }
        //         }

        //         if scanner_matches.len() >= 6 { // Found a pair and orientation that matches!);
        //         // if scanner_matches.len() >= 12 { // Found a pair and orientation that matches!);
        //             println!("COMPARE MATCHED!");
        //             break 'outer;
        //         }

        //         two_vec.rotate_left(1);  // Move vertices over by 1 -- try a different point as "origin".
        //     }
        // // }
        scanner_matches
    }

    // Right to up -- about the Z -- Around X, 90deg -- Y to the right
    // let mut clockwise = || beacon_list.iter_mut().for_each(|tup| std::mem::swap(&mut tup.0, &mut tup.1));
    // let mut flip_clock = || beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.1 *= -1; });
    // let mut anti_clock = || beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.1); tup.0 *= -1; tup.1 *= -1; });
    fn clockwise(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.1); tup.1 *= -1; }); }
    fn flip_clock(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.1 *= -1; }); }
    fn anti_clock(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.1); tup.0 *= -1; }); }

    // Right to back -- about the Y -- Around Y, 90deg -- X to the back
    // let mut spin = || beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.2); tup.2 *= -1; });
    // let mut flip_spin = || beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.2 *= -1 });
    // let mut anti_spin = || beacon_list.iter_mut().for_each(|tup| std::mem::swap(&mut tup.0, &mut tup.2));
    fn spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.2); tup.2 *= -1; }); }
    fn flip_spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { tup.0 *= -1; tup.2 *= -1 }); }
    fn anti_spin(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.0, &mut tup.2); tup.0 *= -1 }); }

    // Front to up -- about the X -- Around Y, 90deg -- Y to the back
    // let mut pitch = || beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.1, &mut tup.2); tup.2 *= -1; });
    // let mut anti_pitch = || beacon_list.iter_mut().for_each(|tup| std::mem::swap(&mut tup.1, &mut tup.2));
    fn pitch(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.1, &mut tup.2); tup.2 *= -1; }); }
    fn anti_pitch(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { std::mem::swap(&mut tup.1, &mut tup.2); tup.1 *= -1; }); }

    #[allow(dead_code)]
    fn flip_pitch(beacon_list: &mut Vec<(i32, i32, i32)>) { beacon_list.iter_mut().for_each(|tup| { tup.1 *= -1; tup.2 *= -1; }); }

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

    #[allow(dead_code)]
    fn rotate_beacons_back(beacon_list: &mut Vec<(i32, i32, i32)>, orientation: i32) {
        // let new_list: Vec<(i32, i32, i32)> = beacon_list;  // Do we even need to clone? Might be able to just edit original.
        match orientation {
            0 => (),  // Original
            1 => anti_clock(beacon_list),
            2 => flip_clock(beacon_list),
            3 => clockwise(beacon_list),

            4 => { anti_spin(beacon_list) }
            5 => { anti_clock(beacon_list); anti_spin(beacon_list) }
            6 => { flip_clock(beacon_list); anti_spin(beacon_list) }
            7 => { clockwise(beacon_list); anti_spin(beacon_list) }

            8 => { flip_spin(beacon_list) }
            9 => { anti_clock(beacon_list); flip_spin(beacon_list) }
            10 => { flip_clock(beacon_list); flip_spin(beacon_list) }
            11 => { clockwise(beacon_list); flip_spin(beacon_list) }

            12 => { spin(beacon_list) }
            13 => { anti_clock(beacon_list); spin(beacon_list) }
            14 => { flip_clock(beacon_list); spin(beacon_list) }
            15 => { clockwise(beacon_list); spin(beacon_list) }

            16 => { anti_pitch(beacon_list) }
            17 => { anti_clock(beacon_list); anti_pitch(beacon_list) }
            18 => { flip_clock(beacon_list); anti_pitch(beacon_list) }
            19 => { clockwise(beacon_list); anti_pitch(beacon_list) }

            20 => { pitch(beacon_list) }
            21 => { anti_clock(beacon_list); pitch(beacon_list) }
            22 => { flip_clock(beacon_list); pitch(beacon_list) }
            23 => { clockwise(beacon_list); pitch(beacon_list) }

            _ => (),
        };
    }

    // let mut matches: Vec<Vec<(i32, i32, i32)>> = vec![];
    // let mut new_beacons: i32 = 0;
    // let mut new_beacons: Vec<(i32, i32, i32)> = input_stuff_vec[0].clone();  // Pre-populate with beacons from scanner 0.
    // let mut scanner_locations: Vec<(i32, i32, i32, i32)> = vec![(0, 0, 0, 0); input_stuff_vec.len()];  // x, y, z, orientation
    // // let one = 0;
    // for one in 0..(input_stuff_vec.len() - 1) {
    //     // println!("SOURCE SCANNER: {}", one);
    //     for two in (one + 1)..input_stuff_vec.len() {
            // let two = 1;
            // input_stuff_vec[one].sort();
            // let one_vec = &input_stuff_vec[one];
            // let mut two_vec: Vec<(i32, i32, i32)> = input_stuff_vec[two].clone();

            // if one_vec.len() != two_vec.len() {
            //     panic!("PROBLEMS with LENGTHS {:?} {:?}", one_vec, two_vec);
            // }

            // println!("ONE_VEC:");
            // for vertex in one_vec { println!("{:?}", vertex); }

    // fn process_two(one: usize, two: usize, input_stuff_vec: &mut Vec<Vec<(i32, i32, i32)>>, new_beacons: &mut Vec<(i32, i32, i32)>, scanner_locations: &mut Vec<(i32, i32, i32, i32)>) {
    fn process_two(one: usize, two: usize, input_stuff_vec: &mut Vec<Vec<(i32, i32, i32)>>, new_beacons: &mut Vec<(i32, i32, i32)>, scanner_locations: &mut Vec<(i32, i32, i32, i32)>) -> usize {
        let mut return_val: usize = 0;

        for or_idx in 0..24 {  // 24 orientations of the beacon list.
            let mut one_vec = input_stuff_vec[one].clone();
            let mut two_vec: Vec<(i32, i32, i32)> = input_stuff_vec[two].clone();  // Manipulate a fresh copy. Rotation starts from same point each time.

            if scanner_locations[one] == (0, 0, 0, 0) && scanner_locations[two] != (0, 0, 0, 0) {
                let temp_vec = one_vec;
                one_vec = two_vec;
                two_vec = temp_vec;
                // std::mem::swap(&mut one_vec, &mut two_vec);
            }

            // println!("\tSOURCE {} TARGET {} ORIENTATION {}", one, two, or_idx);
            rotate_beacons(&mut two_vec, or_idx);
            // two_vec.make_contiguous().sort();

            // println!("TWO_VEC:");
            // for vertex in &two_vec { println!("{:?}", vertex); }

            let mut scanner_two_offset = (0, 0, 0);
            let mut new_matches: Vec<(i32, i32, i32)> = compare_beacons(&one_vec, &mut two_vec, &mut scanner_two_offset);
            // if new_matches.len() >= 6 {
            if new_matches.len() >= 12 {
                // let mut scanner_two_offset: (i32, i32, i32) = offsets;
                // if one != 0 && two != 1 {
                //     scanner_two_offset = (
                //         scanner_locations[one].0 - offsets.0,
                //         scanner_locations[one].1 - offsets.1,
                //         scanner_locations[one].2 - offsets.2,
                //     );
                // }
                // let mut offset_wrapper: Vec<(i32, i32, i32)> = vec![scanner_two_offset];
                // rotate_beacons(&mut offset_wrapper, or_idx);  // Reusing the rotation for the offset, as well.
                // scanner_two_offset = offset_wrapper[0];
                // let scanner_two_offset: (i32, i32, i32) = (offsets.0 * -1, offsets.1 * -1, offsets.2 * -1);  // Location direction is opposite of offset.
                // let mut adjusted_beacons: Vec<(i32, i32, i32)> = vec![];

                // if scanner_locations[two] == (0, 0, 0, 0) {  // Only if not already set. -- already checked on driver.
                    // if one != 0 {
                    //     let mut offset_wrapper: Vec<(i32, i32, i32)> = vec![scanner_two_offset];
                    //     rotate_beacons(&mut offset_wrapper, scanner_locations[one].3);  // Reusing the rotation for the offset, as well.
                    //     scanner_two_offset = offset_wrapper[0];
                    // }
                    scanner_locations[two] = (scanner_locations[one].0 - scanner_two_offset.0,
                                        scanner_locations[one].1 - scanner_two_offset.1,
                                        scanner_locations[one].2 - scanner_two_offset.2,
                                        or_idx
                                        );
                    // scanner_locations[two] = (scanner_two_offset.0, scanner_two_offset.1, scanner_two_offset.2, or_idx);
                    // scanner_locations[two] = (scanner_two_offset.0 * -1, scanner_two_offset.1 * -1, scanner_two_offset.2 * -1, or_idx);  // Location direction is opposite of offset.
                    // scanner_locations[two] = (scanner_locations[one].0 + scanner_two_offset.0,
                    //                     scanner_locations[one].1 + scanner_two_offset.1,
                    //                     scanner_locations[one].2 + scanner_two_offset.2,
                    //                     or_idx
                    //                     );
                    println!("MATCHES! SCANNER {} AND {} OFFSET {:?} OR_IDX {} ADJ: {:?}", one, two, scanner_two_offset, or_idx, scanner_locations[two]);

                    // Reorientate the second vector to be 0-based to make future lookup and comparison results also 0-based.
                    // input_stuff_vec[two].iter_mut().for_each(|x| *x = (x.0 + scanner_locations[two].0, x.1 + scanner_locations[two].1, x.2 + scanner_locations[two].2));
                    // input_stuff_vec[two].iter_mut().for_each(|x| *x = (x.0 + scanner_two_offset.0, x.1 + scanner_two_offset.1, x.2 + scanner_two_offset.2));
                    input_stuff_vec[two] = two_vec;

                    // adjusted_beacons = two_vec.iter().map(|x| (x.0 + scanner_locations[two].0, x.1 + scanner_locations[two].1, x.2 + scanner_locations[two].2)).collect();
                    // two_vec.iter_mut().for_each(|x| *x = (x.0 + scanner_locations[two].0, x.1 + scanner_locations[two].1, x.2 + scanner_locations[two].2));
                    // new_matches.iter_mut().for_each(|x| *x = (x.0 + scanner_locations[two].0, x.1 + scanner_locations[two].1, x.2 + scanner_locations[two].2));
                    // two_vec.iter_mut().for_each(|x| *x = (x.0 + scanner_two_offset.0, x.1 + scanner_two_offset.1, x.2 + scanner_two_offset.2));
                    // new_matches.iter_mut().for_each(|x| *x = (x.0 - scanner_two_offset.0, x.1 - scanner_two_offset.1, x.2 - scanner_two_offset.2));

                    return_val = two;

                    let not_matched: Vec<(i32, i32, i32)> = input_stuff_vec[two].iter().filter(|x| !new_matches.contains(x)).cloned().collect();
                    // new_beacons.append(&mut not_matched);
                    not_matched.iter().for_each(|x| if !new_beacons.contains(x) { new_beacons.push(*x); });
                    // new_beacons += (two_vec.len() - new_matches.len()) as i32;
                    break;
                // }
                // else if one != 0 && scanner_locations[one] == (0, 0, 0, 0) {
                //     // let mut scanner_one_location = (scanner_two_offset.0 * -1, scanner_two_offset.1 * -1, scanner_two_offset.2 * -1);
                //     println!("BEFORE ROTATION: {:?} {}", scanner_two_offset, scanner_locations[two].3);
                //     // // scanner_locations[one] = scanner_one_location;
                    
                //     // let mut offset_wrapper: Vec<(i32, i32, i32)> = vec![scanner_one_location];
                //     // rotate_beacons_back(&mut offset_wrapper, or_idx);  // Reusing the rotation for the offset, as well.
                //     // // rotate_beacons(&mut offset_wrapper, or_idx);  // Reusing the rotation for the offset, as well.
                //     // rotate_beacons(&mut offset_wrapper, scanner_locations[two].3);  // Reusing the rotation for the offset, as well.
                //     // // rotate_beacons(&mut offset_wrapper, 10);  // Reusing the rotation for the offset, as well.
                //     // flip_pitch(&mut offset_wrapper);
                //     // scanner_one_location = offset_wrapper[0];

                //     // scanner_locations[one] = (scanner_one_location.0 + scanner_locations[two].0,
                //     //     scanner_one_location.1 + scanner_locations[two].1,
                //     //     scanner_one_location.2 + scanner_locations[two].2,
                //     //     or_idx
                //     //     );
                //     // scanner_locations[one] = (scanner_locations[two].0 - scanner_one_location.0,
                //     //     scanner_locations[two].1 - scanner_one_location.1,
                //     //     scanner_locations[two].2 - scanner_one_location.2,
                //     //     or_idx
                //     //     );
                //     if one != 0 {
                //         let mut offset_wrapper: Vec<(i32, i32, i32)> = vec![scanner_two_offset];
                //         rotate_beacons(&mut offset_wrapper, scanner_locations[two].3);  // Reusing the rotation for the offset, as well.
                //         // flip_spin(&mut offset_wrapper);  // Looks hacky?
                //         scanner_two_offset = offset_wrapper[0];
                //     }
                //     scanner_locations[one] = (scanner_locations[two].0 - scanner_two_offset.0,
                //         scanner_locations[two].1 - scanner_two_offset.1,
                //         scanner_locations[two].2 - scanner_two_offset.2,
                //         or_idx
                //         );
                //     // println!("MATCHES! SCANNER {} AND {} OFFSET {:?} OR_IDX {} ADJ: {:?}", one, two, scanner_two_offset, or_idx, scanner_locations[one]);

                //     // adjusted_beacons = two_vec.iter().map(|x| (x.0 + scanner_locations[one].0, x.1 + scanner_locations[one].1, x.2 + scanner_locations[one].2)).collect();
                //     two_vec.iter_mut().for_each(|x| *x = (x.0 + scanner_locations[one].0, x.1 + scanner_locations[one].1, x.2 + scanner_locations[one].2));
                //     new_matches.iter_mut().for_each(|x| *x = (x.0 + scanner_locations[one].0, x.1 + scanner_locations[one].1, x.2 + scanner_locations[one].2));

                //     return_val = one;
                // }

                // Display.
                // println!("TWO_VEC");
                // two_vec.sort();
                // two_vec.iter().for_each(|x| println!("{:?}", x));
                // println!("NEW_MATCHES");
                // new_matches.sort();
                // new_matches.iter().for_each(|x| println!("{:?}", x));
                // println!("OFFSETS");
                // scanner_locations.iter().for_each(|x| println!("{:?}", x));
                // println!();

                // // let adjusted_beacons: Vec<(i32, i32, i32)> = two_vec.iter().map(|x| (x.0 + scanner_two_offset.0, x.1 + scanner_two_offset.1, x.2 + scanner_two_offset.2)).collect();
                // // let mut not_matched: Vec<(i32, i32, i32)> = adjusted_beacons.iter().filter(|x| !new_matches.contains(x)).cloned().collect();
                // let not_matched: Vec<(i32, i32, i32)> = two_vec.iter().filter(|x| !new_matches.contains(x)).cloned().collect();
                // // new_beacons.append(&mut not_matched);
                // not_matched.iter().for_each(|x| if !new_beacons.contains(x) { new_beacons.push(*x); });
                // // new_beacons += (two_vec.len() - new_matches.len()) as i32;
                // break;
            }
        }

        return_val
    }

            // matches.sort();
            // matches.iter().for_each(|line| line.iter().for_each(|x| println!("{:?}", x));
            // for (scan_idx, scan_matches) in matches.iter().enumerate() {
            //     println!("SCANNER {}", scan_idx);
            //     scan_matches.iter().for_each(|x| println!("{:?}", x));
            // }
    //     }
    // }

    let mut new_beacons: Vec<(i32, i32, i32)> = input_stuff_vec[0].clone();  // Pre-populate with beacons from scanner 0.
    let mut scanner_locations: Vec<(i32, i32, i32, i32)> = vec![(0, 0, 0, 0); input_stuff_vec.len()];  // x, y, z, orientation

    // for one in 0..(input_stuff_vec.len() - 1) {
    //     // println!("SOURCE SCANNER: {}", one);
    //     for two in (one + 1)..input_stuff_vec.len() {
    //         process_two(one, two, &mut input_stuff_vec, &mut new_beacons, &mut scanner_locations);
    //     }
    // }

    // let work_stack: Vec<i32> = (0..input_stuff_vec.len()).collect();
    
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
    println!("NUM {}", new_beacons.len());
}