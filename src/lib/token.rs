fn tokenize(data: String) -> Vec<[Option<char>; 3]>{
    // token vec structure
    let mut tokens: Vec<[Option<char>; 3]> = vec![];

    let mut nbr_tokens:f32;
    nbr_tokens = data.len() as f32 / 3.0;
    nbr_tokens = nbr_tokens.ceil();

    // add empty tokens structure
    for _ in 0..nbr_tokens as i8{
        tokens.push([None, None, None]);
    }

    // put char in tokens
    let mut i: usize = 0;
    let mut j: usize = 0;

    for char in data.chars(){
        tokens[j][i] = Some(char);
        i +=1; 

        if i > 2{
            i = 0;
            j += 1;
        }
    }

    return tokens;
}


fn untokenize(tokens: Vec<[Option<char>; 3]>) -> String{
    let mut data: String = String::new();

    // travel tokens vec 
    for token in tokens{
        for char in token{
            // push token char to data
            if char == None{
                ;
            }

            else{
                data.push(char.unwrap());
            }
        }
    }

    return data;
}
