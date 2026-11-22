// import
use std::fs::{self, File};
use std::io::{BufReader, prelude::*, Result};


pub fn read_file(file_path: String) -> Result<String> {
    // load objet
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut content = String::new();

    for line in reader.lines() {
        for char in line?.chars() {
            // add chat to final content
            content.push(char);
        }

        // add line retout
        content += "\\n";
    }

    // return Result Ok + content
    Ok(content)
}


pub fn write_file(mut content:String, output_file: String) -> Result<()>{
    // replace \n
    content = content.replace("\\n","\n");

    // write
    fs::write(output_file, content)?;

    // return Ok
    Ok(())
}



/* 
let file_path: String = String::from("mod.rs");
let content: String = read_file(file_path).expect("Erreur");

println!("{}", content);


// write
write_file(
    content, 
    String::from("output.txt")
);
*/