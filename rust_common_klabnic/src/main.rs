fn main() {
    let spaces = "     ";
    let len_spaces = spaces.len();
    println!("{len_spaces}");

    println!("This is tuple");
    let tupple = (500, 53.58, 1234i64);

    println!("{}", tupple.1);
    println!("{}", tupple.0);

    println!("This is an array");
    let arr = [1, 2, 3, 4, 5,6,7,8,9,10,11,12];
    let month: [&str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];

    println!("{}", arr[0]);
    println!("{}", month[0]);

    
    println!("Array Element Access. Pleae input an arras index?");

    let mut index = String::new();
    std::io::stdin().read_line(&mut index).unwrap();
    let index: usize = index.trim().parse().unwrap();
    
    println!("{}", arr[index]);
    println!("{}", month[index]);

    println!("Statements do not return values. Is a statement");

    let condition = true;
    let number = if condition {4} else {6};
    println!("{}", number);

    let mut count = 0;

    loop {
        println!("{}", count);
        count += 1;
        if count > 10 {
            break count * 12;
        }
    };
    println!("The result is {}", count);

    
}
