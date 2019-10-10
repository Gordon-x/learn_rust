use std::fs;

pub fn check_bytes() {
    let f = fs::read("NewFile.txt");
    match f {

        Ok(v) => {
            let mut next = 0;
            let mut char_list:Vec<u8> = vec![];
            for i in v {
                let mut ones = 0;
                let mut tmp = i << 1;
                while tmp.leading_zeros() == 0 {
                    tmp = tmp << 1;
                    ones += 1;
                }
                if next > 0 {
                    next -= 1;
                    char_list.push(i);
                    if next == 0 {
                        println!("utf8 {}", String::from_utf8_lossy(char_list.as_slice()));
                    }
                } else {
                    if ones > 0 {
                        char_list = vec![];
//                        println!("utf8 start {:X}", i);
                        char_list.push(i);
                    } else {
                        println!("ascii {}", char::from(i));
                    }

                    next = ones
                }

            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}