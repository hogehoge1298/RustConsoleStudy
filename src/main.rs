extern crate RPGBattle;

use RPGBattle::Human::Human;


fn main() {
    let mut p1 = Human::new("ツナシ・タクト");

    println!("{}", p1.get_name());
    p1.apprivoiser();
    println!("{}", p1.get_name());

}
