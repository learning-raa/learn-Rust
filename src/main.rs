use std::io;

fn main() {
    println!("rust in..");
    //
    
    println!("enter:");
    let mut textIn = String::new();
    
    io::stdin()
        .read_line(&mut textIn)
        .expect("ERRerr");
    println!("{}", textIn);
    
    //
    println!("..rust out!");
}
