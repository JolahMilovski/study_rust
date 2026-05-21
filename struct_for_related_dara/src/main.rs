#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // Метод-конструктор (ассоциированная функция)
    fn build(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Метод для вывода параметров пользователя (принимает &self по ссылке, чтобы не забирать владение)
    fn print_params(&self) {
        println!(" {} : {} ", self.username, self.email)
    }

    // Метод для обновления email (принимает &mut self, так как изменяет данные)
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

fn main() {
    let mut user_1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let (second_user_name, second_user_email) = (
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    // Используем метод build вместо функции build_user
    let second_user = User::build(second_user_email, second_user_name);

    let third_user = User {
        active: true,
        username: second_user.username, // ВНИМАНИЕ: здесь происходит перемещение владения
        email: String::from("thirduser@user.com"),
        sign_in_count: 1,
    };

    // Используем метод update_email вместо прямого присваивания
    user_1.update_email(String::from("anotheremail@example.com"));

    // Используем метод print_params вместо функции print_user_param
    user_1.print_params();

    // Если нужно вывести third_user (раскомментируйте):
    println!("Third user: {:?}", third_user);
}
