use std::{thread,time};


pub fn runThread () {
    println!("runThread in..");
    //
    thread::spawn( move || { enterThread(); } );
    for i in 0..50 {
        println!("main thread tac: {}", i);
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
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


