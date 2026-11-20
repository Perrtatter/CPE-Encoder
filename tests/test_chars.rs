fn main(){
    let string: String = String::from("Hello");

    println!("{:?}", string.chars());

    for char in string.chars(){
        println!("{}", char);
    }
}