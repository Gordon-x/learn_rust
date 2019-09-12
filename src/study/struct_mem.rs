use std::mem;
use std::slice;
use std::str;

#[derive(Debug)]
pub struct Rect<'a>{
    pub width: u8,    // 1
    pub name: &'a str,
    pub height: [i64; 4],       // 2
}


pub fn struct_mem() {

    let rect = Rect{width: 1, height:[1,2,3,4], name:"33"};

    // &str mem  16  => FatPtr
    let story = "Once";
    let ptr = story.as_ptr();
    let len = story.len();
    unsafe  {
        let slice = slice::from_raw_parts(ptr, len);
        let s = str::from_utf8(slice);
        println!("{:?}", s);
    }

    println!("{:?}", mem::size_of::<*const str>());
    println!("mem Rect:{}", mem::size_of::<Rect>());

    println!("mem rect:{}", mem::size_of_val(&rect));
    println!("mem name:{}", mem::size_of_val(rect.name));
    println!("mem width:{}", mem::size_of_val(&(rect.width)));
    println!("mem height:{}", mem::size_of_val(&(rect.height)));

    println!("rect start {:p}", &rect);
    println!("name:{:p}", &(rect.name));
    println!("width:{:p}", &(rect.width));
    println!("height:{:p}", &(rect.height));
}
