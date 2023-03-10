use std::io;

use crate::not_main::*;
mod not_main;

fn main() {
    println!("[rust in..]");
    //
    
    testMacros();
    runThread();

    println!("enter:");
    let mut textIn = String::new();
    
    io::stdin()
        .read_line(&mut textIn)
        .expect("ERRerr");
    println!("{}", textIn);
    process( &textIn );
    
    ownExample( textIn );
    
    textIn = String::from("new one!!!!!!111");
    println!("own back: {}", textIn);
    
    //
    println!("[..rust out!]");
}

fn process( txt: &String ) {
    println!("txt: {}", txt);
    for item in txt.chars() {
        println!("item: {}", item);
    }
}

fn ownExample( txt: String ) {
    println!("reowned: {}", txt);
}

