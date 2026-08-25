use colored::Colorize;
use recall::load_xp;

pub fn xp() {
    let xp = load_xp();
    println!("{}", format!(" XP: {}", xp.xp).cyan())
}
