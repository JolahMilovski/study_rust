fn main() {
       
    let s1 = String::from("Hello");
    
    let s2 = s1.clone();
        
    let s3 = give_ownership();

    let s4 = String::from("World");

    let s5 = takes_and_gives_back(&s2);    

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    println!("s5: {}", s5);

    //MUTABLE reference
    let mut t = String::from("Accepted");
    
    change(&mut t); 
    println!("t: {}", t);

    let mut net = String::from("Da_da_da");

//это нормальное поведение потому что указатели не мутабельны
    let r_1 = &net;
    let r_2 = &net;
    println!("r1 = {r_1} & r2 = {r_2}");
//это нормальное поведение потому что r_1 и r_2 указывают на один и тот же объект и дальше не используются
    let r_3 = &net;
    println!("r3 = {r_3}");
    
//Срезы НЕ ВЛАДЕЮТ данными ТОЛЬКО заимствуют
    let new_slice = String::from("Farararara");
    let slice = &new_slice[0..2];
    println!("slice: {}", slice);
//ниже будет паника потому что границы среза выходят за пределы строки а проверябтся ТОЛЬКО при выполнении среза
    let slice2 = &new_slice[3..15];
    println!("slice2: {}", slice2); 
    

    
    

    
}

fn change(some_string: &mut String) {
    some_string.push_str(", string");
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(s: &String) -> String {
    s.clone()
}

