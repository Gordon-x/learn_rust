use std::io::stdin;

fn main() {
    let mut input  = String::new();

//    if let Ok(string_bytes) = stdin().read_line(&mut input) {
//        println!("{}, {}", string_bytes, input);
//    } else {
//        println!("error");
//    }

    let res = stdin().read_line(&mut input);
    match res {
        Ok(v) => println!("{}, {}", v,  input),
        Err(e) => println!("{}", e)
    }
}