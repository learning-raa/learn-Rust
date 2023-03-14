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
        assert!(true, "true was false");
    }
    #[test]
    fn bettaFall(){
        assert!(false, "got false!");
    }
    #[test]
    #[should_panic]
    fn letsPanic(){
        panic!("a bit panicue");
    }
}
