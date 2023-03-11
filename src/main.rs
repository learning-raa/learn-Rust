use std::io;

use crate::macro_test::*;
mod macro_test;

use crate::thread_test::*;
mod thread_test;

use crate::owning_test::*;
mod owning_test;



fn main() {
    println!("[rust in..]");
    //
    println!(" 1 - testMacros");
    println!(" 2 - runThread");
    println!(" 3 - testOwning");
    println!("enter task tag:");
    
    let mut textIn = String::new();
    io::stdin()
        .read_line(&mut textIn)
        .expect("ERRerr");
    
    let len = textIn.len() - 1;
    match &textIn[..len] {
        "1" => testMacros(),
        "2" => runThread(),
        "3" => testOwning(),
        _ => println!("unknown tag. exit"),
    }
    
    
    //
    println!("[..rust out!]");
}

