pub struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        if num < 0 {
            return "".to_string();
        }
        let mut num: u32 = num as u32;
        let mut res = String::new();
        let I_int = num % 10;
        match I_int {
            4 => {
                res.push_str("IV");
            }
            9 => {
                res.push_str("IX");
            }
            0..=3 => {
                res.push_str("I".repeat(I_int as usize).as_str());
            }
            5 => {
                res.push('V');
            }
            6..=8 => {
                res.push('V');
                res.push_str("I".repeat((I_int - 5) as usize).as_str());
            }
            _ => {
                unreachable!()
            }
        }
        num /= 10;
        if num == 0 {
            return res;
        }
        let I_int = num % 10;

        let mut tmp_res = String::new();
        match I_int {
            4 => {
                tmp_res.push_str("XL");
            }
            9 => {
                tmp_res.push_str("XC");
            }
            0..=3 => {
                tmp_res.push_str("X".repeat(I_int as usize).as_str());
            }
            5 => {
                tmp_res.push('L');
            }
            6..=8 => {
                tmp_res.push('L');
                tmp_res.push_str("X".repeat((I_int - 5) as usize).as_str());
            }
            _ => {
                unreachable!()
            }
        }
        res = tmp_res + &res;
        num /= 10;
        if num == 0 {
            return res;
        }
        let I_int = num % 10;
        let mut tmp_res = String::new();

        match I_int {
            4 => {
                tmp_res.push_str("CD");
            }
            9 => {
                tmp_res.push_str("CM");
            }
            0..=3 => {
                tmp_res.push_str("C".repeat(I_int as usize).as_str());
            }
            5 => {
                tmp_res.push('D');
            }
            6..=8 => {
                tmp_res.push('D');
                tmp_res.push_str("C".repeat((I_int - 5) as usize).as_str());
            }
            _ => {
                unreachable!()
            }
        }
        res = tmp_res + &res;
        num /= 10;
        let tmp_res = "M".to_string().repeat(num as usize);
        res = tmp_res + &res;
        res
    }
}

// impl Solution {
//     pub fn int_to_roman(num: i32) -> String {
//         let roman_table =[
//             (1000, "M"),
//             (900, "CM"),
//             (500, "D"),
//             (400, "CD"),
//             (100, "C"),
//             (90, "XC"),
//             (50, "L"),
//             (40, "XL"),
//             (10, "X"),
//             (9, "IX"),
//             (5, "V"),
//             (4, "IV"),
//             (1, "I"),
//         ];
//         let mut num = num;
//         let mut roman = String::new();
//         for (key, value) in roman_table {
//             while num >= key {
//                 num -= key;
//                 roman.push_str(value);
//             }
//             if num == 0 {break}
//         }
//         roman
//     }

// }
