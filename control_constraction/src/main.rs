use std::io;
use strum::{IntoEnumIterator};
use strum_macros::{EnumString, Display, EnumIter};

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, Display, EnumString, EnumIter, PartialEq)]
enum UsState {       
    California,
    Texas,
    Florida,
    NewYork,
    Illinois,
    Pennsylvania,
    Ohio,
    Georgia,
    NorthCarolina,
    Michigan,    
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn value_in_cents_plus_one(x: Option<Coin>) -> Option<i32> {
    match x {
        Some(coin) =>  Some((value_in_cents(coin) + 1) as i32),
        None => None,
    }
}



fn main() {

    println!("Enter <number> <state>");
    
    
    let mut input = &mut String::new();
    // Автоматический join всех вариантов
    let all_states: Vec<String> = UsState::iter().map(|s| s.to_string()).collect();
    
    println!("All states: {}", all_states.join(", "));
    
    io::stdin().read_line(&mut input).expect("in read");
    
    let two_parts: Vec<&str> = input.trim().split_whitespace().collect();

    if two_parts.len() != 2 {
        eprintln!("Error: need input number and state");
        return;
    }
    
    let num_coin: i32 = match two_parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a number", two_parts[0]);
            return;
        }
    };

    let state = match two_parts[1] {
        "California" => UsState::California,
        "Texas" => UsState::Texas,
        "Florida" => UsState::Florida,
        "NewYork" => UsState::NewYork,
        "Illinois" => UsState::Illinois,
        "Pennsylvania" => UsState::Pennsylvania,
        "Ohio" => UsState::Ohio,
        "Georgia" => UsState::Georgia,
        "NorthCarolina" => UsState::NorthCarolina,
        "Michigan" => UsState::Michigan,
        _ => {
            eprintln!("Error: unknown state '{}'", two_parts[1]);
            return;
        }
    };
    
    match value_in_cents_plus_one(Some(Coin::Quarter(state))) {
        None => println!("No coin"),
        Some(x) => println!("Penny plus one equals and some else {} cents", x * num_coin),
    }
    
}
