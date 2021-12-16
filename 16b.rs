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

    // https://codereview.stackexchange.com/questions/228867/converting-a-hexadecimal-string-to-a-binary-string-using-rust-pattern-matching-l
    // fn convert_to_binary_from_hex(hex: &str) -> String {
    //     hex[2..].chars().map(to_binary).collect()
    // }

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


    // let mut to_process: Vec<String> = vec![input_str];
    // let mut total = 0;
    // while !to_process.is_empty() {  // Was originally going to do it with loops, but realized recursion might be better.usrlib

    fn process_packet(packet: &String, in_total: &mut i64) -> String {
        // let ver_str: String = packet.chars().take(3).collect::<Vec<char>>().iter().collect();
        // let ver_num = isize::from_str_radix(&ver_str, 2).unwrap();
        // println!("VER: {} {}", ver_num, packet);
        // *in_total += ver_num as i32;

        let type_str: String = packet[3..6].to_string();
        let type_num = isize::from_str_radix(&type_str, 2).unwrap();
        // println!("TYPE: {}", type_num);

        if type_num == 4 {  // Literal value, look for the final 5 starting with a 0.
            let mut all_strings: String = "".to_string();
            let mut start = 6;
            let mut last_starting_bit = '1';
            while last_starting_bit == '1' {
                last_starting_bit = packet[start..(start + 1)].chars().next().unwrap();
                let group = &packet[(start + 1)..(start + 5)];  // &str?
                // println!("LEAD: {} GROUP: {}", last_starting_bit, group);
                all_strings.push_str(&group);
                start += 5;
            }
            // println!("LIT: {} {}", all_strings, isize::from_str_radix(&all_strings, 2).unwrap());

            *in_total = isize::from_str_radix(&all_strings, 2).unwrap() as i64;  // Want to use this number!
            // println!("LIT: {}", in_total);

            return packet[start..].to_string();
        }
        else {

            // To be used later, depending on the operation being performed.
            fn operation(operands: &Vec<i64>, type_num: isize) -> i64 {
                if type_num == 0 {
                    return operands.iter().fold(0, |acc, x| acc + x);
                }
                else if type_num == 1 {
                    return operands.iter().fold(1, |acc, x| acc * x);
                }
                else if type_num == 2 {
                    return *operands.iter().min().unwrap();
                }
                else if type_num == 3 {
                    return *operands.iter().max().unwrap();
                }
                else if type_num == 5 {
                    if operands[0] > operands[1] {
                        return 1;
                    }
                    else {
                        return 0;
                    }
                }
                else if type_num == 6 {
                    if operands[0] < operands[1] {
                        return 1;
                    }
                    else {
                        return 0;
                    }
                }
                else {
                    if operands[0] == operands[1] {
                        return 1;
                    }
                    else {
                        return 0;
                    }
                }
            }

            let length_type = packet[6..7].chars().next().unwrap();
            if length_type == '0' {  // Char size determines number of subpackets.
                let subpacket_length_str = &packet[7..22];
                let subpacket_length_num: i32 = isize::from_str_radix(&subpacket_length_str, 2).unwrap() as i32;
                let subpacket = &packet[22 as usize..(22 + subpacket_length_num) as usize].to_string();
                // println!("SP: {} {}", subpacket_length_num, subpacket);

                let mut new_string = subpacket.clone();
                let mut subtotal: Vec<i64> = vec![];
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
                // println!("SP: {} {}", subpacket_num_num, subpacket);

                let mut new_string = subpacket.clone();
                let mut iter = 0;
                let mut subtotal: Vec<i64> = vec![];
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
    let mut new_string = input_str;
    // while new_string.find("1").is_some()  {
        new_string = process_packet(&new_string, &mut total);
    // }
    println!("TOTAL: {} {}", total, new_string);
}