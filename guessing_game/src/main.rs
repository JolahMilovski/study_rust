use std::io;


fn main() {
    
    
    println!("Guess a number!");

    let secret_number = rand::random_range(1..=100);

        loop {
            
            
            // для ввода с клавиатуры создаем изменяемую переменную, т.к. будем в нее записывать данные
            let mut guess = String::new();      
            
            
            match io::stdin()
                .read_line(&mut guess){
                    Ok(_) => {

                        //парсинг и обработка данных с ввода
                        let guess:i32 = match guess.trim().parse() {
            
                                Ok(num) => num,
                                Err(_) => {
                                    println!("This is not a number");
                                    continue
                                },
            
                            };
                        
                        println!("You pass is {}", guess);
                        
                        
                        
                        match guess.cmp(&secret_number) {
                            std::cmp::Ordering::Less => println!("Too small!"),
                            std::cmp::Ordering::Equal => {
            
                                println!("my secret number is {}", secret_number);
                                println!("You WIN!!");
                                break
                            }
                            std::cmp::Ordering::Greater => println!(" Too big"),        
                        }
            
                        println!("Guess a number!");

                    },
                    Err(_) => continue
                }
            
        }   
    
}
