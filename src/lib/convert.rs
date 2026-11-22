// import 
use std::collections::HashMap;
use std::sync::LazyLock;


// init the variable with Lazylock
static BYTES_CHAR_MAP: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    HashMap::from([
        // digits
        ('0', 48), ('1', 49), ('2', 50), ('3', 51), ('4', 52), ('5', 53), ('6', 54), ('7', 55), ('8', 56), ('9', 57),

        // letters lower
        ('a', 97), ('b', 98), ('c', 99), ('d', 100), ('e', 101), ('f', 102), ('g', 103), ('h', 104), ('i', 105), ('j', 106), ('k', 107), ('l', 108), ('m', 109), ('n', 110), ('o', 111), ('p', 112), ('q', 113), ('r', 114), ('s', 115), ('t', 116), ('u', 117), ('v', 118), ('w', 119), ('x', 120), ('y', 121), ('z', 122),

        // letters upper
        ('A', 65), ('B', 66), ('C', 67), ('D', 68), ('E', 69), ('F', 70), ('G', 71), ('H', 72), ('I', 73), ('J', 74), ('K', 75), ('L', 76), ('M', 77), ('N', 78), ('O', 79), ('P', 80), ('Q', 81), ('R', 82), ('S', 83), ('T', 84), ('U', 85), ('V', 86), ('W', 87), ('X', 88), ('Y', 89), ('Z', 90),

        // punctuations
        ('!', 33), ('"', 34), ('#', 35), ('$', 36), ('%', 37), ('&', 38), ('\'', 39), ('(', 40), (')', 41), ('*', 42), ('+', 43), (',', 44), ('-', 45), ('.', 46), ('/', 47), (':', 58), (';', 59), ('<', 60), ('=', 61), ('>', 62), ('?', 63), ('@', 64), ('[', 91), ('\\', 92), (']', 93), ('^', 94), ('_', 95), ('`', 96), ('{', 123), ('|', 124), ('}', 125), ('~', 126),

        // special
        (' ', 32), ('\t', 9), ('\n', 10), ('\r', 13), ('\x0b', 11), ('\x0c', 12),
    ])
});


pub fn encode(tokens: Vec<[Option<char>; 3]>) -> Vec<[u8; 3]> {
    let mut pixel_list: Vec<[u8; 3]> = vec![];

    // travel tokens
    for token in tokens {
        let mut pixel = [0u8; 3];
        
        for (i, char_opt) in token.iter().enumerate() {
            if let Some(c) = char_opt {
                if let Some(&byte_code) = BYTES_CHAR_MAP.get(c) {
                    pixel[i] = byte_code;
                }
            }
        }

        pixel_list.push(pixel);
    }

    return pixel_list;
}


fn decode(pixel_list: Vec<[u8; 3]>) -> Vec<[Option<char>; 3]> {
    let mut tokens: Vec<[Option<char>; 3]> = vec![];

    for pixel in pixel_list {
        let mut token: [Option<char>; 3] = [None; 3];

        for (i, &rgb) in pixel.iter().enumerate() {
            for (&utf8_char, &utf8_num) in BYTES_CHAR_MAP.iter() {
                if rgb == utf8_num {
                    token[i] = Some(utf8_char);
                    break;
                }
            }
        }

        tokens.push(token);
    }

    return tokens;
}