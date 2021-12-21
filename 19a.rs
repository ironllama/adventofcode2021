pub mod usrlib;

use std::collections::VecDeque;

fn main() {
    let input_stuff = [
        "--- scanner 0 ---",
        "404,-588,-901",
        "528,-643,409",
        "-838,591,734",
        "390,-675,-793",
        "-537,-823,-458",
        "-485,-357,347",
        "-345,-311,381",
        "-661,-816,-575",
        "-876,649,763",
        "-618,-824,-621",
        "553,345,-567",
        "474,580,667",
        "-447,-329,318",
        "-584,868,-557",
        "544,-627,-890",
        "564,392,-477",
        "455,729,728",
        "-892,524,684",
        "-689,845,-530",
        "423,-701,434",
        "7,-33,-71",
        "630,319,-379",
        "443,580,662",
        "-789,900,-551",
        "459,-707,401",
        "",
        "--- scanner 1 ---",
        "686,422,578",
        "605,423,415",
        "515,917,-361",
        "-336,658,858",
        "95,138,22",
        "-476,619,847",
        "-340,-569,-846",
        "567,-361,727",
        "-460,603,-452",
        "669,-402,600",
        "729,430,532",
        "-500,-761,534",
        "-322,571,750",
        "-466,-666,-811",
        "-429,-592,574",
        "-355,545,-477",
        "703,-491,-529",
        "-328,-685,520",
        "413,935,-424",
        "-391,539,-444",
        "586,-435,557",
        "-364,-763,-893",
        "807,-499,-711",
        "755,-354,-619",
        "553,889,-390",
        "",
        "--- scanner 2 ---",
        "649,640,665",
        "682,-795,504",
        "-784,533,-524",
        "-644,584,-595",
        "-588,-843,648",
        "-30,6,44",
        "-674,560,763",
        "500,723,-460",
        "609,671,-379",
        "-555,-800,653",
        "-675,-892,-343",
        "697,-426,-610",
        "578,704,681",
        "493,664,-388",
        "-671,-858,530",
        "-667,343,800",
        "571,-461,-707",
        "-138,-166,112",
        "-889,563,-600",
        "646,-828,498",
        "640,759,510",
        "-630,509,768",
        "-681,-892,-333",
        "673,-379,-804",
        "-742,-814,-386",
        "577,-820,562",
        "",
        "--- scanner 3 ---",
        "-589,542,597",
        "605,-692,669",
        "-500,565,-823",
        "-660,373,557",
        "-458,-679,-417",
        "-488,449,543",
        "-626,468,-788",
        "338,-750,-386",
        "528,-832,-391",
        "562,-778,733",
        "-938,-730,414",
        "543,643,-506",
        "-524,371,-870",
        "407,773,750",
        "-104,29,83",
        "378,-903,-323",
        "-778,-728,485",
        "426,699,580",
        "-438,-605,-362",
        "-469,-447,-387",
        "509,732,623",
        "647,635,-688",
        "-868,-804,481",
        "614,-800,639",
        "595,780,-596",
        "",
        "--- scanner 4 ---",
        "727,592,562",
        "-293,-554,779",
        "441,611,-461",
        "-714,465,-776",
        "-743,427,-804",
        "-660,-479,-426",
        "832,-632,460",
        "927,-485,-438",
        "408,393,-506",
        "466,436,-512",
        "110,16,151",
        "-258,-428,682",
        "-393,719,612",
        "-211,-452,876",
        "808,-476,-593",
        "-575,615,604",
        "-485,667,467",
        "-680,325,-822",
        "-627,-443,-432",
        "872,-547,-609",
        "833,512,582",
        "807,604,487",
        "839,-516,451",
        "891,-625,532",
        "-652,-548,-490",
        "30,-46,-14",
    ];

    let mut input_stuff_vec: Vec<Vec<(i32, i32, i32)>> = vec![];
    let mut scanner_vec = vec![];
    for idx in 0..input_stuff.len() {
        if input_stuff[idx].is_empty() {
            input_stuff_vec.push(scanner_vec);
            scanner_vec = vec![];
        }
        else if input_stuff[idx].chars().collect::<Vec<char>>()[0] != '-' {
            let new_coords_vec: Vec<i32> = input_stuff[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            let new_coords: (i32, i32, i32) = (new_coords_vec[0], new_coords_vec[1], new_coords_vec[2]);
            scanner_vec.push(new_coords);
        }
    }
    input_stuff_vec.push(scanner_vec);  // Don't forget the last set!
    // println!("INPUT: {:?}", input_stuff_vec);

    // for one in 0..(input_stuff_vec.len() - 1) {
    //     for two in (one + 1)..input_stuff_vec.len() {
            let one = 0;
            let two = 1;
            let one_vec = &input_stuff_vec[one];
            let mut two_vec: VecDeque<(i32, i32, i32)> = VecDeque::from(input_stuff_vec[two].clone());

            if one_vec.len() != two_vec.len() {
                panic!("PROBLEMS with LENGTHS");
            }

            // let mut matches = 0;
            let mut matches: Vec<(i32, i32, i32)> = vec![];
            for dos_rotate in 0..two_vec.len() {
                two_vec.rotate_left(dos_rotate);
                
                let mut offsets = (0, 0, 0);

                for dos in 0..two_vec.len() {
                    let uno_start: (i32, i32, i32) = one_vec[dos];
                    let dos_start: (i32, i32, i32) = two_vec[dos];
                    println!("STARTS: {:?} {:?}", uno_start, dos_start);

                    if dos == 0 {
                        offsets = (uno_start.0 - dos_start.0, uno_start.1 - dos_start.1, uno_start.2 - dos_start.2);
                        println!("OFFSET: {:?}", offsets);
                        // matches += 1;
                        matches.push(uno_start);
                    }
                    else {
                        if (dos_start.0 - offsets.0) == uno_start.0
                            && (dos_start.1 - offsets.1) == uno_start.1
                            && (dos_start.2 - offsets.2) == uno_start.2
                        {
                            // matches += 1;
                            matches.push(uno_start);
                        }
                    }
                }

                if matches.len() >= 12 { // Found a pair and orientation that matches! matches);
                    break;
                }
            }

            // if matches >= 12 {

            // }
            // println!("MATCHES {:?}", matches);
            matches.sort();
            matches.iter().for_each(|x| println!("{:?}", x));
    //     }
    // }
}