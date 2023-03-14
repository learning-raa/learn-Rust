#![allow(non_snake_case)]


pub fn testTest() {
    println!("testTest in..");
    //
    
    //
    println!("..testMacros out!");
}


#[cfg(test)]
mod test {

    #[test]
    fn alphaTest(){
        println!("--> in alpha testOR");
    }
}
