#![allow(non_snake_case)]

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

pub fn testCursive() {
    println!("[cursive in..]");
    //
    let mut curRoot = cursive::default();
    curRoot.add_global_callback('q', |s| s.quit());

    curRoot.add_layer(
        Dialog::around(TextView::new("dlg AROUND"))
            .title("rustUI")
            .button("show Info", |s| showInfo(s) )
            .button("var 2", |s| showVar2(s) )
    );
    curRoot.run();
    //
    println!("[..cursive out!]");
}

fn showInfo(s: &mut Cursive) {
    s.add_layer(Dialog::info("Some info"));
}

fn showVar2(s: &mut Cursive) {
    s.add_layer(
        Dialog::text("dlg.text")
            .title("dlg TITLE")
            .button("press Me", |s| s.add_layer(Dialog::info("nothing")) )
    );
}

