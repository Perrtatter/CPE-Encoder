// basic colors
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const RESET: &str = "\x1b[0m";

// advance
pub const FATAL: &str = "[\x1b[31m!\x1b[0m]";
pub const WARNING: &str = "[\x1b[33m!\x1b[0m]";
pub const INFO: &str = "[\x1b[34m*\x1b[0m]";
pub const SUCCESS: &str = "[\x1b[32m*\x1b[0m]";



/*
mod color;

fn main() {
    println!("{}Hello", color::RED);
    println!("{}Hello", color::BLUE);
    println!("{}Hello", color::GREEN);
    println!("{}Hello", color::YELLOW);
    println!("{}Hello", color::CYAN);
    println!("{}Hello", color::MAGENTA);
    println!("{}Hello", color::RESET);

    println!("");

    println!("{} Hello", color::FATAL);
    println!("{} Hello", color::WARNING);
    println!("{} Hello", color::SUCCESS);
    println!("{} Hello", color::INFO);

}
*/
