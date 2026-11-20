use std::collections::HashMap;


fn main(){
    // create hashmap
    let mut fruits_sell:HashMap<String, i8> = HashMap::from([
        (String::from("Apple"), 50)
    ]);

    // insert 
    fruits_sell.insert(String::from("Peer"), 10);

    // print 
    println!("{}", fruits_sell.get("Peer").unwrap());

    // print all keys
    for (fruit, nbr_sell) in fruits_sell{
        println!("Fruit: {}, Number: {}", fruit, nbr_sell);
    }
}