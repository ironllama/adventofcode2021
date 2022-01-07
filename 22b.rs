pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     // "on x=0..10,y=0..10,z=0..10",
    //     // "off x=9..11,y=9..11,z=9..11",
    //     // "on x=6..12,y=6..12,z=6..12",

    //     // "on x=10..12,y=10..12,z=10..12",
    //     // "on x=11..13,y=11..13,z=11..13",
    //     // "off x=9..11,y=9..11,z=9..11",
    //     // "on x=10..10,y=10..10,z=10..10",
    //     // // "on x=12..12,y=12..12,z=12..12",
    //     // // "on x=11..12,y=11..12,z=11..12",
    //     // // "off x=19..19,y=19..19,z=19..19",
    //     // // "off x=8..10,y=11..11,z=12..12",

    //     // "on x=-20..47,y=-22..23,z=-26..28",

    //     // "on x=-20..26,y=-36..17,z=-47..7",
    //     // "on x=-20..33,y=-21..23,z=-26..28",
    //     // "on x=-22..28,y=-29..23,z=-38..16",
    //     // "on x=-46..7,y=-6..46,z=-50..-1",
    //     // "on x=-49..1,y=-3..46,z=-24..28",
    //     // "on x=2..47,y=-22..22,z=-23..27",  // !
    //     // "on x=-27..23,y=-28..26,z=-21..29",
    //     // "on x=-39..5,y=-6..47,z=-3..44",
    //     // "on x=-30..21,y=-8..43,z=-13..34",
    //     // "on x=-22..26,y=-27..20,z=-29..19",  // !
    //     // "off x=-48..-32,y=26..41,z=-47..-37",
    //     // "on x=-12..35,y=6..50,z=-50..-2",
    //     // "off x=-48..-32,y=-32..-16,z=-15..-5",
    //     // "on x=-18..26,y=-33..15,z=-7..46",
    //     // "off x=-40..-22,y=-38..-28,z=23..41",
    //     // "on x=-16..35,y=-41..10,z=-47..6",
    //     // "off x=-32..-23,y=11..30,z=-14..3",
    //     // "on x=-49..-5,y=-3..45,z=-29..18",
    //     // "off x=18..30,y=-20..-8,z=-3..13",
    //     // "on x=-41..9,y=-7..43,z=-33..15",

    //     "on x=-5..47,y=-31..22,z=-19..33",
    //     "on x=-44..5,y=-27..21,z=-14..35",
    //     "on x=-49..-1,y=-11..42,z=-10..38",
    //     "on x=-20..34,y=-40..6,z=-44..1",
    //     "off x=26..39,y=40..50,z=-2..11",
    //     "on x=-41..5,y=-41..6,z=-36..8",
    //     "off x=-43..-33,y=-45..-28,z=7..25",
    //     "on x=-33..15,y=-32..19,z=-34..11",
    //     "off x=35..47,y=-46..-34,z=-11..5",
    //     "on x=-14..36,y=-6..44,z=-16..29",
    //     "on x=-57795..-6158,y=29564..72030,z=20435..90618",
    //     "on x=36731..105352,y=-21140..28532,z=16094..90401",
    //     "on x=30999..107136,y=-53464..15513,z=8553..71215",
    //     "on x=13528..83982,y=-99403..-27377,z=-24141..23996",
    //     "on x=-72682..-12347,y=18159..111354,z=7391..80950",
    //     "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709",
    //     "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856",
    //     "on x=-52752..22273,y=-49450..9096,z=54442..119054",
    //     "on x=-29982..40483,y=-108474..-28371,z=-24328..38471",
    //     "on x=-4958..62750,y=40422..118853,z=-7672..65583",
    //     "on x=55694..108686,y=-43367..46958,z=-26781..48729",
    //     "on x=-98497..-18186,y=-63569..3412,z=1232..88485",
    //     "on x=-726..56291,y=-62629..13224,z=18033..85226",
    //     "on x=-110886..-34664,y=-81338..-8658,z=8914..63723",
    //     "on x=-55829..24974,y=-16897..54165,z=-121762..-28058",
    //     "on x=-65152..-11147,y=22489..91432,z=-58782..1780",
    //     "on x=-120100..-32970,y=-46592..27473,z=-11695..61039",
    //     "on x=-18631..37533,y=-124565..-50804,z=-35667..28308",
    //     "on x=-57817..18248,y=49321..117703,z=5745..55881",
    //     "on x=14781..98692,y=-1341..70827,z=15753..70151",
    //     "on x=-34419..55919,y=-19626..40991,z=39015..114138",
    //     "on x=-60785..11593,y=-56135..2999,z=-95368..-26915",
    //     "on x=-32178..58085,y=17647..101866,z=-91405..-8878",
    //     "on x=-53655..12091,y=50097..105568,z=-75335..-4862",
    //     "on x=-111166..-40997,y=-71714..2688,z=5609..50954",
    //     "on x=-16602..70118,y=-98693..-44401,z=5197..76897",
    //     "on x=16383..101554,y=4615..83635,z=-44907..18747",
    //     "off x=-95822..-15171,y=-19987..48940,z=10804..104439",
    //     "on x=-89813..-14614,y=16069..88491,z=-3297..45228",
    //     "on x=41075..99376,y=-20427..49978,z=-52012..13762",
    //     "on x=-21330..50085,y=-17944..62733,z=-112280..-30197",
    //     "on x=-16478..35915,y=36008..118594,z=-7885..47086",
    //     "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456",
    //     "off x=2032..69770,y=-71013..4824,z=7471..94418",
    //     "on x=43670..120875,y=-42068..12382,z=-24787..38892",
    //     "off x=37514..111226,y=-45862..25743,z=-16714..54663",
    //     "off x=25699..97951,y=-30668..59918,z=-15349..69697",
    //     "off x=-44271..17935,y=-9516..60759,z=49131..112598",
    //     "on x=-61695..-5813,y=40978..94975,z=8655..80240",
    //     "off x=-101086..-9439,y=-7088..67543,z=33935..83858",
    //     "off x=18020..114017,y=-48931..32606,z=21474..89843",
    //     "off x=-77139..10506,y=-89994..-18797,z=-80..59318",
    //     "off x=8476..79288,y=-75520..11602,z=-96624..-24783",
    //     "on x=-47488..-1262,y=24338..100707,z=16292..72967",
    //     "off x=-84341..13987,y=2429..92914,z=-90671..-1318",
    //     "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188",
    //     "off x=-27365..46395,y=31009..98017,z=15428..76570",
    //     "off x=-70369..-16548,y=22648..78696,z=-1892..86821",
    //     "on x=-53470..21291,y=-120233..-33476,z=-44150..38147",
    //     "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("22.in.txt");

    let mut input_stuff_vec: Vec<(&str, Vec<Vec<i32>>)> = vec![];
    for line_num in 0..input_stuff.len() {
        let line = &input_stuff[line_num];
        if let Some((on_off, coords)) = line.split_once(" ") {
            // Split up each line into tokens and extract the range numbers.
            let mut axes: Vec<Vec<i32>> = vec![];
            coords.splitn(3, ',').for_each(|xyz| {
                if let Some((_axis, values)) = xyz.split_once("=") {
                    let mut ranges: Vec<i32> = values.split("..").map(|num_str| num_str.parse().unwrap()).collect();
                    ranges.sort();
                    axes.push(ranges);
                }
            });
            input_stuff_vec.push((on_off, axes));
        }
    }
    // println!("STUFF_VEC: {:?}", input_stuff_vec);


    fn slice_and_dice(axis: usize, loc: i32, cuboid: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
        // println!("slice_and_dice: {} {} {:?}", axis, loc, cuboid);
        let mut one: Vec<Vec<i32>> = cuboid.clone();
        let mut two: Vec<Vec<i32>> = cuboid.clone();

        if cuboid[axis][0] == loc {
            one[axis][1] = loc;
            two[axis][0] = loc + 1;
        }
        else if cuboid[axis][1] == loc {
            one[axis][1] = loc - 1;
            two[axis][0] = loc;
        }
        else {
            one[axis][1] = loc - 1;
            // one[axis][1] = if loc > one[axis][0] {loc - 1} else {loc};  // Don't want to shrink beyond the original borders.
            two[axis][0] = loc;
        }

        one[axis].sort();
        two[axis].sort();

        return vec![one, two];
        // println!("SLICE: {:?} @ {} --> {:?} - {:?}", cuboid, loc, one, two);
    }

      // If cuboid one completely envelopes cuboid two, then we return a true.
    fn cuboid_contains(one: &Vec<Vec<i32>>, two: &Vec<Vec<i32>>) -> bool {
        if one[0][0] <= two[0][0] && one[0][1] >= two[0][1]
            && one[1][0] <= two[1][0] && one[1][1] >= two[1][1]
            && one[2][0] <= two[2][0] && one[2][1] >= two[2][1] {
            return true;
        }
        return false;
    }

    // Tests if for a specific axis, two is wholly contained within one.
    fn axis_contains(one: &Vec<i32>, two: &Vec<i32>) -> bool {
        if one[0] <= two[0] && one[1] >= two[1] {
            return true;
        }
        return false;
    }

    // Check for intersections. Returning (axis, location to cut). A return of (4,x) means one completely envelopes the other.
    fn intersects(one: &Vec<Vec<i32>>, two: &Vec<Vec<i32>>) -> Option<(usize, i32)> {
        if cuboid_contains(&one, &two) {  // If one wholly envelopes two, send back the special code.
            // println!("intersects: EVELOPES.");
            return Some((4, 2));
        }

        // Else, we find out where to cut up cuboid two.
        let mut inter_loc: (usize, i32) = (4, 0);
        let mut intersection: u8 = 0;
        for axis in (0..3).rev() {  // Check all 3 axes for intersection!
            // print!("intersects: [{}] one{:?} <=> two{:?}: ", axis, one[axis], two[axis]);
            // Use edges of cuboid one to find a slice plane for cuboid two.
            if one[axis][0] <= two[axis][0] && one[axis][1] >= two[axis][0] {  // Two's first point within range of one?
                if !axis_contains(&one[axis], &two[axis]) {
                    inter_loc = (axis, one[axis][1]);
                }
                intersection += 1;
                // println!("...TRUE[0] - two inside one.");
            }
            else if one[axis][0] <= two[axis][1] && one[axis][1] >= two[axis][1] {  // Two's second point within range of one?
                if !axis_contains(&one[axis], &two[axis]) {
                    inter_loc = (axis, one[axis][0]);
                }
                intersection += 1;
                // println!("...TRUE[1] - two inside one.");
            }
            else if one[axis][0] > two[axis][0] && one[axis][1] < two[axis][1] {  // Two larger than one? Keep checking others.
                inter_loc = (axis, one[axis][0]);
                intersection += 1;
                // println!("...TRUE[2] - one inside two.");
            }
            else {
                // println!("...FALSE");
            }
        }
        if intersection >= 3 && inter_loc != (4, 0) {
            // println!("intersects: FOUND: {:?}", inter_loc);
            return Some(inter_loc);
        }
        return None;
    }

    let mut all_cuboids: Vec<Vec<Vec<i32>>> = vec![input_stuff_vec[0].1.clone()];  // List of cuboids B, described as 3 lists of axes values.
    for a_cuboid in &input_stuff_vec {  // For each cuboid A to be processed... Gonna try to insert wholesale.
        // println!("STARTING {:?}", a_cuboid);
        let mut b_add_cubes = vec![];  // List of chopped up B pieces after processing against all A's.

        for idx_all in 0..all_cuboids.len() {  // ...check against all past added cuboids B's. This is the running list of new cuboids.
            // Take out one from the B's and potentially chop it up.
            let mut new_cubes: Vec<Vec<Vec<i32>>> = vec![all_cuboids[idx_all].clone()];  // Gonna chop up one B. Temporary basket/queue to store chopped up pieces.
            while let Some(cutme_cuboid) = new_cubes.pop() {  // Check basket of pieces from one B for intersections with current A.
                if let Some((axis, loc)) = intersects(&a_cuboid.1, &cutme_cuboid) {
                    if axis != 4 {  // Only slice/dice if it is not wholly contained. If it is, just ignore it so it disappears.
                        let loc_adj = if loc == a_cuboid.1[axis][0] || loc == a_cuboid.1[axis][1] {loc} else {loc + 1};
                        let temp_cubes: Vec<Vec<Vec<i32>>> = slice_and_dice(axis, loc_adj, &cutme_cuboid);
                        // println!("TEMP_CUBES: {:?}", temp_cubes);
                        if !temp_cubes.is_empty() {
                            new_cubes.extend(temp_cubes);  // Put results back into basket to further chop up.
                            // println!("NEW_CUBES: {:?}", new_cubes);
                        }
                    }
                    else {
                        // println!("ENVELOPES {:?}. RESULT {:?} NEXT.", (axis, loc), cutme_cuboid);
                    }
                }
                else {
                    // println!("NO INTERSECTION. ADDED {:?}. NEXT.", cutme_cuboid);
                    b_add_cubes.push(cutme_cuboid);  // Finished chopping up this piece of B.
                }
            }
        }
            all_cuboids = b_add_cubes;
            if a_cuboid.0 == "on" {
                all_cuboids.push(a_cuboid.1.clone());
            }
        // println!("CURR all_cuboids: {:?}", all_cuboids);
        // Process next cuboid A.
    }

    // println!("RESULT LEN: {:?}", all_cuboids.len());
    // println!("RESULT:");
    // all_cuboids.iter().for_each(|cuboid| println!("{:?}", cuboid));
    // println!();
    let total_volume: i64 = all_cuboids.iter().fold(0, |acc, cuboid|
        acc +
            (((cuboid[0][1] - cuboid[0][0]) + 1) as i64  // The +1 because the range is inclusive.
            * ((cuboid[1][1] - cuboid[1][0]) + 1) as i64
            * ((cuboid[2][1] - cuboid[2][0]) + 1) as i64));
    println!("TOTAL: {}", total_volume);
}



    // Will have to reverse A's and B's for "off". Should have thought about this sooner, as doing it this way will work for both on and off.
    // Below only works for on. ㅠㅠ
    // let mut all_cuboids: Vec<Vec<Vec<i32>>> = vec![input_stuff_vec[0].1.clone()];  // List of cuboids, described as 3 lists of axes values.
    // for line in &input_stuff_vec {  // For each cuboid A to be processed...
    //     let mut add_cubes = vec![];  // List of chopped up pieces after processing against all B's.

    //     for idx_all in 0..all_cuboids.len() {  // ...check against all past added cuboids B's. This is the running list of new cuboids.
    //         let cuboid: &Vec<Vec<i32>> = &all_cuboids[idx_all];  // One previously-added cuboid B for comparison.
    //         let mut new_cubes: Vec<Vec<Vec<i32>>> = vec![line.1.clone()];  // Gonna chop up A. Temporary basket to store chopped up pieces.
    //         while let Some(cutme_cuboid) = new_cubes.pop() {  // Check basket of A pieces for intersections with current B.
    //             if let Some((axis, loc)) = intersects(&cuboid, &cutme_cuboid) {
    //                 if axis != 4 {  // Only slice/dice if it is not wholly contained. If it is, just ignore it so it disappears.
    //                     let temp_cubes: Vec<Vec<Vec<i32>>> = slice_and_dice(axis, loc, &cutme_cuboid);
    //                     // println!("TEMP_CUBES: {:?}", temp_cubes);
    //                     if !temp_cubes.is_empty() {
    //                         new_cubes.extend(temp_cubes);  // Put results back into basket.
    //                         println!("NEW_CUBES: {:?}", new_cubes);
    //                     }
    //                 }
    //                 else {
    //                     println!("ENVELOPES {:?}. RESULT {:?} NEXT.", (axis, loc), cutme_cuboid);
    //                 }
    //             }
    //             else {
    //                 add_cubes.push(cutme_cuboid);  // A's that don't intersect go back into the list for future comparison w/ B's.
    //             }
    //         }
    //         // add_cubes.extend(new_cubes);  // Store this group of chopped A's against one particular B.
    //     }
    //     all_cuboids.extend(add_cubes);
    //     // Process next cuboid A.
    // }


        //     {
        //         let mut axis_and_ranges: Vec<Vec<i32>> = vec![];
        //         for idx in 0..xyzs.len() {
        //             let coords = xyzs[idx];
        //             let coords_tokens: Vec<&str> = coords.split("=").collect();
        //             let mut ranges: Vec<i32> = coords_tokens[1].split("..").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
        //             ranges.sort();  // Lower always first.
        //                 axis_and_ranges.push((ranges[0]..=ranges[1]).collect::<Vec<i32>>());
        //         }
        //     }
        // }

        // // Split up each line into tokens and extract the range numbers.
        // let xyzs: Vec<&str> = tokens[1].split(',').collect();
        // let mut axis_and_ranges: Vec<Vec<i32>> = vec![];
        // for idx in 0..xyzs.len() {
        //     let coords = xyzs[idx];
        //     let coords_tokens: Vec<&str> = coords.split("=").collect();
        //     let mut ranges: Vec<i32> = coords_tokens[1].split("..").map(|num_str| num_str.parse::<i32>().unwrap()).collect();
        //     ranges.sort();  // Lower always first.
        //         axis_and_ranges.push((ranges[0]..=ranges[1]).collect::<Vec<i32>>());
        // }
    // }

    // println!("VEC: {:?}", core);


    //     if tokens[0] == "on" {  // If on, turn the point into a 1.
    //         for x in &axis_and_ranges[0] {
    //             for y in &axis_and_ranges[1] {
    //                 for z in &axis_and_ranges[2] {
    //                     // core[*x as usize][*y as usize][*z as usize] = 1;
    //                     core[(x + hsize) as usize][(y + hsize) as usize][(z + hsize) as usize] = 1;
    //                 }
    //             }
    //         }
    //     }
    //     else {  // If off, turn the point into a 0.
    //         for x in &axis_and_ranges[0] {
    //             for y in &axis_and_ranges[1] {
    //                 for z in &axis_and_ranges[2] {
    //                     // core[*x as usize][*y as usize][*z as usize] = 0;
    //                     core[(x + hsize) as usize][(y + hsize) as usize][(z + hsize) as usize] = 0;
    //                 }
    //             }
    //         }
    //     }
    // }

    // // Count the number of points that have 1s
    // let mut num_on = 0;
    // for x in 0..core.len() {
    //     for y in 0..core[x].len() {
    //         for z in 0..core[x][y].len() {
    //             if core[x][y][z] == 1 {
    //                 num_on += 1;
    //                 // println!("{} {} {}", x, y, z);
    //             }
    //         }
    //     }
    // }

    // println!("NUM ON {}", num_on);