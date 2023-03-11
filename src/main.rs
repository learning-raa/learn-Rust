use std::io;

//use crate::not_main::*;
//mod not_main;

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
    
    match &textIn[..len] {
        "1" => println!("the ONE"),
        _ => println!("others"),
    }
    
    println!("entered:<{}>", textIn);
    
        
    //
    println!("[..rust out!]");
}

