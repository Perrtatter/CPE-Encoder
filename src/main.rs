// import
use std::env;
use std::process::Output;
mod lib;
use lib::cli_utils;
use lib::convert;
use lib::token;


fn main() {
    // arguments
    let args: Vec<String> = env::args().collect();

    // no arguments
    if args.len() == 1{
        println!("No arguments found !");
        println!("Help : ./main --help")
    }

    // show help
    let help_tag:String = String::from("--help");

    if args.contains(&help_tag){
        println!("{}", cli_utils::HELP);
    }

    // show version
    let version_tag:String = String::from("--version");

    if args.contains(&version_tag){
        println!("{}", cli_utils::VERSION);
    }

    // encode/decode
    let encode_tag:String = String::from("--encode");
    let decode_tag:String = String::from("--decode");
    let output_tag:String = String::from("--ouput");

    if args.contains(&encode_tag) || args.contains(&decode_tag){
        if !args.contains(&output_tag){
            println!("{} Fatal: please use the output flag (--help for help) !", cli_utils::FATAL)
        }
    }
}
