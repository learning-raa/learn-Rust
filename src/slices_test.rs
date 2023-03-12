

pub fn testSlices() {
    println!("[slices in..]");
    //
    let originalText = String::from("hello, slices");
    unownString(&originalText);
    unownSlice(&originalText[..1]);
    unownSlice(&originalText);
    //
    let originalArray = [1,3,146];
    unownArray(&originalArray);
    unownArray(&originalArray[..2]);
    //
    println!("[..slices out!]");
}

fn unownString(txt:&String) {
    println!("fn arg: {}", txt);
}
fn unownSlice(txt:&str) {
    println!("fn slice: {}", txt);
}

fn unownArray(arr:&[i32]){
    println!("--");
    for item in arr {
        println!("-- item: {}", item);
    }
}
