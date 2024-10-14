// Importing dependencies
use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};
use std::process::Command;

fn main() {
    // Create a new Cursive root
    let mut siv = Cursive::default();

    // let output = Command::new("cmd")
    // .args(&["/c", "dir"])
    // .output()
    // .expect("Failed to execute command!");

    let output = Command::new("cmd")
    .args(&["/c", "echo HOW TO PIPE EXTERNAL COMMANDS FROM CURRENT PATH !!"])
    .output()
    .expect("Failed to execute command!");

    let output_str = String::from_utf8_lossy(&output.stdout);

    siv.add_layer(
        Dialog::around(TextView::new(output_str))
        .title("Command Output")
        .button("Quit", |s|s.quit())
    );
    siv.run();
}
