

pub fn testOwning() {
    println!("[owning in..]");
    //
    let v1 = DropTest{tag: String::from("v1")};
    let v2 = MetaStruct { 
        alpha: DropTest{tag: String::from("v2.alpha")}, betta: DropTest{tag: String::from("v2.betta")} 
    };
    //
    println!("[..owning out!]");
}


struct MetaStruct {
    alpha: DropTest,
    betta: DropTest,
}


struct DropTest {
    tag: String,
}

impl Drop for DropTest {
    fn drop(&mut self) {
        println!(" -- drop DropTest <{}> -- ", self.tag);
    }
}
