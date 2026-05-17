use std::io;

fn add(x: i32, y: i32) -> i32 {
    let z = x + y;
    return z;
}
fn subtract(x: i32, y: i32) -> i32 {
    let z = x - y;
    return z;
}
fn multiply(x: i32, y: i32) -> i32 {
    let z = x * y;
    return z;
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num1).expect("Ошибка чтения");
    let num1: i32 = num1.trim().parse().expect("Ошибка преобразования");
    io::stdin().read_line(&mut num2).expect("Ошибка чтения");
    let num2: i32 = num2.trim().parse().expect("Ошибка преобразования");
    println!("{}", add(num1, num2));
    println!("{}", subtract(num1, num2));
    println!("{}", multiply(num1, num2));
}
