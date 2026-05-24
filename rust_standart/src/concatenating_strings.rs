///clone string
pub fn by_cloning() {
    let hello = "Hello ".to_string();
    let world = "world".to_string();

    let hello_world = hello.clone() + &world;
    println!("{}", hello_world)    
}
///add str to string
pub fn by_mutation() {

    let mut hello = "Hello ".to_string();
    let world = "world".to_string();

    hello.push_str(&world);
    println!("{}",hello);
}