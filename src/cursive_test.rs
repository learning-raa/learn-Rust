use cursive::views::{Dialog, TextView};

pub fn testCursive() {
    println!("[cursive in..]");
    //
    let mut curRoot = cursive::default();
    curRoot.add_global_callback('q', |s| s.quit());

    curRoot.add_layer(
        Dialog::around(TextView::new("wwOww"))
            .title("rustUI")
            .button("let Quit", |s| println!("!!!!!!!!!!!!!!!") )
    );
    curRoot.run();
    //
    println!("[..cursive out!]");
}


