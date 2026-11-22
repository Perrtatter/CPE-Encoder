// basic colors
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";

// advance
pub const FATAL: &str = "[\x1b[31m!\x1b[0m]";
pub const INFO: &str = "[\x1b[34m*\x1b[0m]";
pub const SUCCESS: &str = "[\x1b[32m*\x1b[0m]";

// cli 
pub const HELP: &str = "
Help menu :
---------------
--help     :  show this menu and quit
--decode   :  decode file given in input (.png)
--encode   :  encode file given in input (text)
--output   :  specified the file in output

Usage :
--------------- 
Encode file :
./main --encode data.txt --output encode.png

Decode file:
./main --decode encode.png --output data.txt
";

pub const VERSION: &str = "
Version : 4.3
By Perrtatter
";

