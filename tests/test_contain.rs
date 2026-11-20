fn main() {
    let fruit: Vec<&str> = vec![
        "Apple",
        "Orange"
    ];

    if fruit.contains(&"Apple"){
        println!("Yes for Apple !");
    }

    if  !fruit.contains(&"Peer"){
        println!("No for Peer !");
    }
}