use std::ops::*;
fn main() {
    let a = 10i32;
    let b = 3;
    let f = 2.5f64;

    println!("Сложение: {}", a.add(1)); // 11
    println!("Вычитание: {}", a.sub(b)); // 7
    println!("Умножение: {}", a.mul(b)); // 30
    println!("Деление: {}", a.div(b)); // 3 (целочисленное деление)
    println!("Остаток: {}", a.rem(b)); // 1
    println!("Степень (целое): {}", a.pow(2)); // 100
    println!("Степень (плавающая точка): {}", f.powf(2.0)); // 6.25

    let x = 5;
    let y = 10;

    println!("Равно: {}", x.eq(&y)); // false
    println!("Не равно: {}", x.ne(&y)); // true
    println!("Меньше: {}", x.lt(&y)); // true
    println!("Меньше или равно: {}", x.le(&y)); // true
    println!("Больше: {}", x.gt(&y)); // false
    println!("Больше или равно: {}", x.ge(&y)); // false

    let min_val = 2;
    let max_val = 5;

    let value = 7;
    println!("Ограничение: {}", value.clamp(min_val, max_val)); // 5 (ограничено max_val)

    let value2 = 3;
    println!("Ограничение: {}", value2.clamp(min_val, max_val)); // 3 (в пределах диапазона)

    let value3 = 1;
    println!("Ограничение: {}", value3.clamp(min_val, max_val)); // 2 (ограничено min_val)

    let num = 3.5f64;
    println!("Rounded num: {}", num);

    let num = 3.7f64;
    let rounded_num = num.floor(); // rounded_num будет 3.0
    println!("{}", rounded_num);

    let num4 = -3.7f64;
    let rounded_num4 = num4.ceil(); // rounded_num4 будет -3.0
    println!("{}", rounded_num4);


    let num4 = -3.7f64;
    let rounded_num4 = num4.trunc(); // rounded_num4 будет -3.0
    println!("{}", rounded_num4);

}