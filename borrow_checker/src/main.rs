// ============================================
// ВЛАДЕНИЕ, ЗАИМСТВОВАНИЕ И ПЕРЕЗАИМСТВОВАНИЕ В RUST
// ============================================

// ===== 1. ВЛАДЕНИЕ (OWNERSHIP) =====
// Присваивание ПЕРЕМЕЩАЕТ (move) владение для типов без Copy

fn ownership_demo() {
    let country = String::from("Austria");
    let country_copy = country; // Владение ПЕРЕШЛО к country_copy
    
    // println!("Country: {}", country); // ❌ Ошибка! country больше не владелец
    println!("Country copy: {}", country_copy); // ✅ работает
}

// ===== 2. ЗАИМСТВОВАНИЕ (BORROWING) — НЕИЗМЕНЯЕМЫЕ ССЫЛКИ &T =====
// Ссылка НЕ забирает владение, можно много таких ссылок

fn borrowing_demo() {
    let city = String::from("Vienna");
    let city_ref = &city; // city_ref: &String — ссылка на city
    
    println!("City: {}", city);       // ✅ city всё ещё владелец
    println!("City ref: {}", city_ref); // ✅ работает через ссылку
    
    // Неизменяемые ссылки копируются (реализуют Copy)
    let ref2 = city_ref;  // копия ссылки
    println!("Both refs: {} and {}", city_ref, ref2); // ✅ обе работают
}

// ===== 3. ВРЕМЯ ЖИЗНИ ССЫЛОК =====
// Ссылка НЕ может жить дольше данных, на которые указывает

fn lifetime_demo() {
    let some_ref: &String;
    {
        let name = String::from("Alice");
        some_ref = &name;
        println!("Inner: {}", some_ref); // ✅ работает внутри блока
    } // name уничтожена
    
    // println!("Outer: {}", some_ref); // ❌ висячий указатель (dangling reference)
}

// Функция НЕ может вернуть ссылку на локальную переменную
/*
fn create_ref() -> &String {
    let greeting = String::from("Good day");
    &greeting // ❌ ошибка: возвращается ссылка на локальную переменную
}
*/

// Правильно: возвращать само значение
fn create_string() -> String {
    let greeting = String::from("Good day");
    greeting // ✅ владение передаётся наружу
}

// ===== 4. ССЫЛКИ КАК ПАРАМЕТРЫ ФУНКЦИЙ =====

fn print_planet(planet_name: &String) { // принимает ссылку
    println!("Planet: {}", planet_name);
}

fn references_as_parameters() {
    let planet = String::from("Earth");
    print_planet(&planet); // передаём ссылку
    println!("Still have planet: {}", planet); // ✅ владение не потеряно
}

// ===== 5. РАЗЫМЕНОВАНИЕ (DEREFERENCE) — ОПЕРАТОР * =====
// Превращает ссылку в значение

fn dereference_demo() {
    let number = 42;
    let number_ref = &number;
    
    // let is_greater = number_ref > 10; // ❌ нельзя сравнить ссылку с числом
    let is_greater = *number_ref > 10; // ✅ * даёт значение
    println!("42 > 10 = {}", is_greater); // true
}

// РАЗЫМЕНОВАНИЕ: Copy vs non-Copy типы

fn dereference_copy_vs_move() {
    // === Copy тип (i32) — создаётся копия ===
    let x = 5;
    let ref_x = &x;
    let y = *ref_x; // y получает КОПИЮ значения
    println!("x: {}, y: {}, ref_x: {}", x, y, ref_x); // ✅ всё работает
    
    // === non-Copy тип (String) — нельзя переместить из-под ссылки ===
    let s = String::from("hello");
    let ref_s = &s;
    // let t = *ref_s; // ❌ Ошибка! нельзя переместить String из-под ссылки
    println!("s: {}, ref_s: {}", s, ref_s); // ✅ можно читать через ссылку
}

// ===== 6. ИЗМЕНЯЕМЫЕ ССЫЛКИ — &mut T =====
// Позволяют изменять данные, на которые указывают

fn change_message(mes: &mut String) { // принимает &mut
    mes.push('!');
}

fn mutable_references_demo() {
    let mut message = "hello".to_string(); // обязательно mut
    println!("До: {}", message);
    
    change_message(&mut message); // передаём &mut
    println!("После: {}", message); // hello!
}

// ОГРАНИЧЕНИЯ ИЗМЕНЯЕМЫХ ССЫЛОК:

fn mutable_references_rules() {
    let mut s1 = "hello".to_string();
    
    // === Правило 1: Только одна &mut в области видимости ===
    let s2 = &mut s1;
    // let s3 = &mut s1; // ❌ вторая изменяемая ссылка
    
    // === Правило 2: Нельзя смешивать &mut и & (если они пересекаются) ===
    let s2 = &mut s1;
    // let s3 = &s1; // ❌ нельзя, потому что s2 (изменяемая) активна
    // println!("{}", s3);
    
    // === Можно, если использовать последовательно ===
    let s3 = &s1;
    println!("{}", s3); // последнее использование неизменяемой ссылки
    let s2 = &mut s1;   // ✅ теперь можно
    s2.push('!');
}

// ===== 7. ПОВЕДЕНИЕ ССЫЛОК ПРИ ПРИСВАИВАНИИ =====
// &T (неизменяемая) — КОПИРУЕТСЯ (реализует Copy)
// &mut T (изменяемая) — ПЕРЕМЕЩАЕТСЯ (не реализует Copy)

fn reference_assignment_demo() {
    // === Неизменяемая ссылка — Copy ===
    let test = String::from("hi");
    let t2 = &test;
    let t3 = t2; // копирование ссылки
    
    println!("t2: {}", t2); // ✅ всё ещё работает
    println!("t3: {}", t3); // ✅ работает
    
    // === Изменяемая ссылка — move ===
    let mut test2 = String::from("hi");
    let t2 = &mut test2;
    let t3 = t2; // владение ссылкой ПЕРЕШЛО к t3
    
    // println!("t2: {}", t2); // ❌ ошибка! t2 больше не владеет ссылкой
    println!("t3: {}", t3); // ✅ работает
}

// ===== 8. ПЕРЕЗАИМСТВОВАНИЕ (REBORROWING) — &mut *ref =====
// Временное "одалживание" изменяемой ссылки

fn foo(r: &mut i32) {
    *r += 1;
}

fn reborrowing_demo() {
    let mut a = 7;
    let b = &mut a;        // b — изменяемая ссылка на a
    let c = &mut *b;       // c — ПЕРЕЗАИМСТВОВАНИЕ у b
    
    *c = 1;                // меняем через c
    println!("b: {}", b);  // ✅ b снова доступна (перезаимствование закончилось)
    // println!("c: {}", c); // ❌ c больше не действует
}

// Перезаимствование при вызове функции (неявное)
fn reborrowing_in_function_call() {
    let mut a = 7;
    let b = &mut a;
    
    foo(b); // Rust неявно делает &mut *b (перезаимствование)
    
    println!("b: {}", b); // ✅ b снова доступна после вызова функции
}

// Цепочка перезаимствований (работает!)
fn reborrowing_chain() {
    let mut a = 7;
    let b = &mut a;
    let c = &mut *b;   // c перезаимствует у b
    let d = &mut *c;   // d перезаимствует у c
    
    *d = 2;
    println!("c: {}", c); // ✅ c снова доступна (после использования d)
    println!("b: {}", b); // ✅ b снова доступна (после использования c)
}

// Нельзя перезаимствовать у НЕИЗМЕНЯЕМОЙ ссылки
/*
fn reborrow_from_immutable_error() {
    let mut a = 2;
    let b = &a;           // неизменяемая ссылка
    let c = &mut *b;      // ❌ ошибка
}
*/

// ===== 9. СВОДНАЯ ТАБЛИЦА =====
/*
| Концепция          | Синтаксис        | Суть                                      |
|--------------------|------------------|-------------------------------------------|
| Ссылка (неизм.)    | &x               | Читаем, не забираем владение              |
| Ссылка (изм.)      | &mut x           | Меняем, одна на область                   |
| Разыменование      | *ref             | Получаем значение из ссылки               |
| Перезаимствование  | &mut *ref        | Временно одалживаем &mut у другой &mut    |
| &T при присв.      | let y = x;       | Копируется (Copy)                         |
| &mut T при присв.  | let y = x;       | Перемещается (move)                       |
*/

// ===== ТРИ ГЛАВНЫХ ПРАВИЛА (запомнить!) =====
/*
1. Ссылка не может жить дольше данных, на которые указывает.
2. Либо ОДНА &mut, либо СКОЛЬКО УГОДНО &, но не вместе (если пересекаются).
3. &mut T при присваивании ПЕРЕМЕЩАЕТСЯ (не копируется).
*/

// ===== ЗАПУСК ВСЕХ ДЕМО =====
fn main() {
    println!("=== 1. Владение ===");
    ownership_demo();
    
    println!("\n=== 2. Заимствование (неизменяемые ссылки) ===");
    borrowing_demo();
    
    println!("\n=== 3. Время жизни ссылок ===");
    lifetime_demo();
    
    println!("\n=== 4. Ссылки как параметры функций ===");
    references_as_parameters();
    
    println!("\n=== 5. Разыменование ===");
    dereference_demo();
    dereference_copy_vs_move();
    
    println!("\n=== 6. Изменяемые ссылки ===");
    mutable_references_demo();
    mutable_references_rules();
    
    println!("\n=== 7. Поведение ссылок при присваивании ===");
    reference_assignment_demo();
    
    println!("\n=== 8. Перезаимствование (Reborrowing) ===");
    reborrowing_demo();
    reborrowing_in_function_call();
    reborrowing_chain();
    
    println!("\n=== 9. Пример с create_string ===");
    let result = create_string();
    println!("result = {}", result);
}
