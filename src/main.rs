extern crate RPGBattle;

use std::thread;
use std::io;
use std::time::Duration;
use RPGBattle::Human::Human;

pub fn clear_screen() -> bool {
    std::process::Command::new("clear").status().unwrap().success()
}


fn main() {
    let mut p1 = Human::new("ツナシ・タクト");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Faild to read line");

    p1.input_action(guess);

    /*if clear_screen() {
        println!("{}", p1.get_name());
    }

    thread::sleep(Duration::from_millis(1000));

    if clear_screen() {
        p1.apprivoiser();
    }

    thread::sleep(Duration::from_millis(1000));

    if clear_screen() {
        println!("{}", p1.get_name());
    }*/


}
