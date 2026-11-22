// import
use std::env;
mod lib;
use lib::cli_utils;
use lib::convert;
use lib::token;
use lib::file;
use lib::png;


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
    let output_tag:String = String::from("--output");
    let mut mode: String = String::new();

    // no output argument
    if args.contains(&encode_tag) || args.contains(&decode_tag){
        if !args.contains(&output_tag){
            println!("{} Fatal: please use the output flag (--help for help) !", cli_utils::FATAL)
        }

        // get input file
        else{
            // choose mod 
            if args.contains(&encode_tag){
                mode = String::from("encoding");
            }

            if args.contains(&decode_tag){
                mode = String::from("decoding");
            }

            // encode
            if mode == "encoding"{
                // get file input
                let input_file: &str = &args[2];
                let input_file: String = String::from(input_file);

                println!("{} File specified: {}{}{}",cli_utils::SUCCESS, cli_utils::GREEN, input_file, cli_utils::RESET);

                // read file 
                let content: String = file::read_file(input_file).expect("Error");
                println!("{} File found !", cli_utils::SUCCESS);

                println!();

                // tokenize
                println!("{} Tokenize ...", cli_utils::INFO);
                let tokens: Vec<[Option<char>; 3]> = token::tokenize(content);

                // convert 
                println!("{} Convert ...", cli_utils::INFO);
                let pixel_list: Vec<[u8; 3]> = convert::encode(tokens);
 
                // get file input
                let output_file: &str = &args[4];
                let output_file: String = String::from(output_file);

                // write png
                println!("{} Write PNG ...", cli_utils::INFO);
                png::write_png(pixel_list, output_file)

                println!();

                println!("{} Encode saved: {}{}{}", cli_utils::SUCCESS, cli_utils::GREEN, output_file, cli_utils::RESET);
            }
        }
    }


}
