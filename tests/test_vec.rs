fn main(){
    let mut vec: Vec<u8> = vec![1,2,3,4,5];
    vec.push(6);

    println!("{:?}", vec);

    let mut matrix: Vec<[u8; 3]> = vec![];
    matrix.push([1,2,3]);

    println!("{:?}", matrix);

    println!("{:?}", matrix.get(0).unwrap().get(1).unwrap());


}