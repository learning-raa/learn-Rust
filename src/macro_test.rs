

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

