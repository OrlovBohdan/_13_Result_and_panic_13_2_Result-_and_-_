#[test]

/*

use std::num::ParseIntError;

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply(n1_str: &str, n2_str: &str) -> __ {
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}
*/
fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12); // Перевірка правильного результату
    println!("Success!");
}

use std::num::ParseIntError;

// Реалізація функції multiply
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?; // Використання ? для обробки помилки
    let n2 = n2_str.parse::<i32>()?; // Використання ? для обробки помилки
    Ok(n1 * n2) // Повертаємо результат множення
}



/*
Тип повернення multiply: Встановлено Result<i32, ParseIntError>, що дозволяє повертати значення або помилку.

Обробка парсингу: Застосовано оператор ? для n1_str.parse::<i32>() і n2_str.parse::<i32>(), що автоматично обробляє помилки, повертаючи їх у випадку невдачі.

Повернення результату: Після успішного парсингу функція повертає результат множення в обгортці Ok.
*/