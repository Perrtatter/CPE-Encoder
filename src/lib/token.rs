pub fn tokenize(data: String) -> Vec<[Option<char>; 3]> {
    let chars: Vec<char> = data.chars().collect();
    let mut tokens: Vec<[Option<char>; 3]> = Vec::new();

    for chunk in chars.chunks(3) {
        let mut token = [None; 3];
        for (i, &char) in chunk.iter().enumerate() {
            token[i] = Some(char);
        }
        tokens.push(token);
    }

    return tokens;
}


pub fn untokenize(tokens: Vec<[Option<char>; 3]>) -> String {
    let mut data: String = String::new();

    for token in tokens {
        for char in token {
            if let Some(char) = char {
                data.push(char);
            }
        }
    }

    return data;
}