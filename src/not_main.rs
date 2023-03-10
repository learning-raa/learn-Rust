use std::{thread,time};


pub fn runThread () {
    println!("runThread in..");
    //
    thread::spawn( move || { enterThread(); } );
    //
    println!("..runThread out!");
}

fn enterThread() {
    println!("ohhhh");
    loop {
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        println!("tic..");
    }
}


macro_rules! runMy {
    ()=>{
        println!("Hohoho");
    };
}

pub fn testMacros() {
    println!("testMacros in..");
    //
    runMy!();
    //
    println!("..testMacros out!");
}

