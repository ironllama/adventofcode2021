pub mod usrlib;

fn main() {
    // let input_stuff = [
    //     // "D2FE28",
    //     // "38006F45291200",
    //     // "EE00D40C823060",
    //     // "8A004A801A8002F478",
    //     // "620080001611562C8802118E34",
    //     // "C0015000016115A2E0802F182340",
    //     // "A0016C880162017C3686B18A3D4780",
    //     // "C200B40A82",
    //     // "04005AC33890",
    //     // "880086C3E88112",
    //     // "CE00C43D881120",
    //     // "D8005AC2A8F0",
    //     // "F600BC2D8F",
    //     // "9C005AC2F8F0",
    //     "9C0141080250320F1802104A08",
    // ];
    let input_stuff = usrlib::vec_lines_from_file("16.in.txt");

    fn to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }

    let input_vec: Vec<&str> = input_stuff[0].chars().map(|x| to_binary(x)).collect();
    let input_str: String = input_vec.iter().map(|x| x.to_string()).collect();
    // println!("STR: {}", input_str);

    fn process_packet(packet: &String, in_total: &mut i64) -> String {

        let type_str: String = packet[3..6].to_string();
        let type_num = isize::from_str_radix(&type_str, 2).unwrap();

        if type_num == 4 {  // Literal value, look for the final 5 starting with a 0.
            let mut all_strings: String = "".to_string();
            let mut start = 6;
            let mut last_starting_bit = '1';
            while last_starting_bit == '1' {
                last_starting_bit = packet[start..(start + 1)].chars().next().unwrap();
                let group = &packet[(start + 1)..(start + 5)];  // &str?
                all_strings.push_str(&group);
                start += 5;
            }

            *in_total = isize::from_str_radix(&all_strings, 2).unwrap() as i64;  // Want to use this number!
            return packet[start..].to_string();
        }
        else {
            // Added for sub-operations. To be used later, depending on the operation being performed.
            fn operation(operands: &Vec<i64>, type_num: isize) -> i64 {
                return match type_num {
                    0 => operands.iter().fold(0, |acc, x| acc + x),
                    1 => operands.iter().fold(1, |acc, x| acc * x),
                    2 => *operands.iter().min().unwrap(),
                    3 => *operands.iter().max().unwrap(),
                    // 4 => Literals!
                    5 => if operands[0] > operands[1] {1} else {0},
                    6 => if operands[0] < operands[1] {1} else {0},
                    7 => if operands[0] == operands[1] {1} else {0},
                    _ => panic!(),  // Should never be called?
                }
            }

            let length_type = packet[6..7].chars().next().unwrap();
            if length_type == '0' {  // Char size determines number of subpackets.
                let subpacket_length_str = &packet[7..22];
                let subpacket_length_num: i32 = isize::from_str_radix(&subpacket_length_str, 2).unwrap() as i32;
                let subpacket = &packet[22 as usize..(22 + subpacket_length_num) as usize].to_string();

                let mut new_string = subpacket.clone();
                let mut subtotal: Vec<i64> = vec![];  // Addded to keep track of sub-operation totals.
                while new_string.find("1").is_some()  {
                    let mut subsubtotal = 0;
                    new_string = process_packet(&new_string, &mut subsubtotal);
                    subtotal.push(subsubtotal);
                }
                *in_total = operation(&subtotal, type_num);  // Removed the +=

                return packet[(22 + subpacket_length_num) as usize..].to_string();
            }
            else if length_type == '1' {
                let subpacket_num_str = &packet[7..18];
                let subpacket_num_num = isize::from_str_radix(&subpacket_num_str, 2).unwrap();
                let subpacket = &packet[18..].to_string();

                let mut new_string = subpacket.clone();
                let mut iter = 0;
                let mut subtotal: Vec<i64> = vec![];  // Addded to keep track of sub-operation totals.
                while iter < subpacket_num_num as i32 && new_string.find("1").is_some()  {
                    let mut subsubtotal = 0;
                    new_string = process_packet(&new_string, &mut subsubtotal);
                    subtotal.push(subsubtotal);
                    iter += 1;
                }
                *in_total = operation(&subtotal, type_num);  // Removed the +=

                return new_string;
            }
            return "BAD".to_string();
        }
    }

    let mut total = 0i64;
    process_packet(&input_str, &mut total);

    println!("TOTAL: {}", total);
}


// Notes:
// 6  0                6  4        2  4
// 110000 100000000010 11010000001 01010000010
// ADD    N2                  1           2

// 0  1                    5  4        3  4
// 000001 0000000000010110 10110000110 01110001001 0000
// MULT   L22                     6           9

// 4  2                    5  4        6  4        0  4
// 100010 0000000000100001 10110000111 11010001000 00010001001 0
// MIN    L33                     7           8           9

// 6  6                    5  4        2  4
// 110110 0000000000010110 10110000101 01010001111 0000
// LESS   L22                     5           15

// 7  5                7  4        5  4
// 111101 100000000010 11110000101 10110001111
// GREAT  N2                  5           15

// 4  7                    5  4        7  4
// 100111 0000000000010110 10110000101 11110001111 0000
// EQUAL  L22                     5           15

// 4  7                    2  0                2  4        4  4        6  1                0  4        2  4
// 100111 0000000001010000 010000 100000000010 01010000001 10010000011 110001 100000000010 00010000010 01010000010 00
// EQUAL  L80              ADD    N2                  1           3    MULT   N2                  2           2
