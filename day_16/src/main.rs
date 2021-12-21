
fn four_bit_string(v : i32) -> String {
    
    let mut s: [char; 4] = ['_';4];

    for i in 0..4 {
        let one = v & (1 << (4 - i - 1)) > 0;
        s[i] = if one { '1' } else { '0' };
    }
    
    s.iter().collect()
}

fn hex_char_to_i32(c : char) -> i32 {
    i32::from_str_radix(&c.to_string(), 16).unwrap()
}

fn bin_to_i32(s: &str) -> i32 {
    i32::from_str_radix(&s, 2).unwrap()
}

// fn parse_packet(bin_str: &str, start_index : i32) -> i32 {
//     let mut it = bin_str.chars();

//     let mut packet_bit_count = 0;

//     let version : String = it.by_ref().take(3).collect();
//     packet_bit_count += 3;
//     let type_id : String = it.by_ref().take(3).collect();
//     packet_bit_count += 3;

//     println!("{}, {}", version, bin_to_i32(&version));
//     println!("{}, {}", type_id, bin_to_i32(&type_id));

//     let is_literal = bin_to_i32(&type_id) == 4;
//     if  is_literal {
//         let mut literal_str = String::from("");
//         loop {
//             let group : String = it.by_ref().take(5).collect();
//             packet_bit_count += 5;
//             literal_str += &group[1..];

//             if group.as_bytes()[0] == '0' as u8 {
//                 break
//             }
//         }
//         let n = bin_to_i32(&literal_str);
//         println!("n = {}", n);
//     } else {
//         let length_type_id = bin_to_i32(&it.by_ref().take(1).collect::<String>());
//         packet_bit_count += 1;
//         if length_type_id == 0 {
//             // the length is a 15-bit number representing the number of bits in the sub-packets.
//             let num_sub_packet_bits = bin_to_i32(&it.by_ref().take(15).collect::<String>());
//             packet_bit_count += 15;
//             loop {
//                 let bits = parse_packet(&it.by_ref().collect::<String>());
//             }
//         } else if length_type_id == 1 {
//             // the length is a 11-bit number representing the number of sub-packets.
//             let num_sub_packets = bin_to_i32(&it.by_ref().take(11).collect::<String>());
//             packet_bit_count += 11;

//         }
//     }

//     packet_bit_count
// }

fn parse_packet(bin_str: &str) -> (String, i32) {
    let mut packet_bit_count = 0;

    let mut it = bin_str.chars();

    let version : String = it.by_ref().take(3).collect();
    packet_bit_count += 3;
    let type_id : String = it.by_ref().take(3).collect();
    packet_bit_count += 3;

    println!("{}, {}", version, bin_to_i32(&version));
    println!("{}, {}", type_id, bin_to_i32(&type_id));

    let is_literal = bin_to_i32(&type_id) == 4;
    if  is_literal {
        let mut literal_str = String::from("");
        loop {
            let group : String = it.by_ref().take(5).collect();
            packet_bit_count += 5;
            literal_str += &group[1..];

            if group.as_bytes()[0] == '0' as u8 {
                break
            }
        }
        let n = bin_to_i32(&literal_str);
        println!("n = {}", n);
    } else {
        let length_type_id = bin_to_i32(&it.by_ref().take(1).collect::<String>());
        packet_bit_count += 1;
        if length_type_id == 0 {
            // the length is a 15-bit number representing the number of bits in the sub-packets.
            let num_sub_packet_bits = bin_to_i32(&it.by_ref().take(15).collect::<String>());
            packet_bit_count += 15;

            let rest = it.by_ref().collect::<String>().clone();

            loop {
                let bits = parse_packet(&rest);
            }
        } else if length_type_id == 1 {
            // the length is a 11-bit number representing the number of sub-packets.
            let num_sub_packets = bin_to_i32(&it.take(11).collect::<String>());
            packet_bit_count += 11;

        }
    }

    packet_bit_count
}

fn main() {
    let sample = "D2FE28";
    let _input = "E20D79005573F71DA0054E48527EF97D3004653BB1FC006867A8B1371AC49C801039171941340066E6B99A6A58B8110088BA008CE6F7893D4E6F7893DCDCFDB9D6CBC4026FE8026200DC7D84B1C00010A89507E3CCEE37B592014D3C01491B6697A83CB4F59E5E7FFA5CC66D4BC6F05D3004E6BB742B004E7E6B3375A46CF91D8C027911797589E17920F4009BE72DA8D2E4523DCEE86A8018C4AD3C7F2D2D02C5B9FF53366E3004658DB0012A963891D168801D08480485B005C0010A883116308002171AA24C679E0394EB898023331E60AB401294D98CA6CD8C01D9B349E0A99363003E655D40289CBDBB2F55D25E53ECAF14D9ABBB4CC726F038C011B0044401987D0BE0C00021B04E2546499DE824C015B004A7755B570013F2DD8627C65C02186F2996E9CCD04E5718C5CBCC016B004A4F61B27B0D9B8633F9344D57B0C1D3805537ADFA21F231C6EC9F3D3089FF7CD25E5941200C96801F191C77091238EE13A704A7CCC802B3B00567F192296259ABD9C400282915B9F6E98879823046C0010C626C966A19351EE27DE86C8E6968F2BE3D2008EE540FC01196989CD9410055725480D60025737BA1547D700727B9A89B444971830070401F8D70BA3B8803F16A3FC2D00043621C3B8A733C8BD880212BCDEE9D34929164D5CB08032594E5E1D25C0055E5B771E966783240220CD19E802E200F4588450BC401A8FB14E0A1805B36F3243B2833247536B70BDC00A60348880C7730039400B402A91009F650028C00E2020918077610021C00C1002D80512601188803B4000C148025010036727EE5AD6B445CC011E00B825E14F4BBF5F97853D2EFD6256F8FFE9F3B001420C01A88915E259002191EE2F4392004323E44A8B4C0069CEF34D304C001AB94379D149BD904507004A6D466B618402477802E200D47383719C0010F8A507A294CC9C90024A967C9995EE2933BA840";

    let binary_str = sample.chars().map(|c| hex_char_to_i32(c)).fold(String::from(""), |out, number| out + &four_bit_string(number));
    println!("{}", binary_str);
    //assert_eq!(binary_str, "110100101111111000101000");

    parse_packet(&binary_str);

    

}
