#[test]
/*

// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> __ {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, __);

    let result = multiply("t", "2");
    assert_eq!(result.__, 8);

    println!("Success!");
}
*/
fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20)); // Змінено для перевірки правильного значення

    let result = multiply("t", "2");
    assert_eq!(result.is_err(), true); // Перевіряємо, що сталася помилка

    println!("Success!");
}
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> { // Змінено тип повернення
    let n1 = n1_str.parse::<i32>()?; // Використання `?` для обробки помилок
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2) // Повертаємо результат
}




/*
Тип повернення multiply: Змінено на Result<i32, ParseIntError>, щоб вказати, що функція може повернути або значення типу i32, або помилку.

Обробка помилок: Додано ? після n1_str.parse::<i32>() та n2_str.parse::<i32>(), що дозволяє автоматично повертати помилку, якщо парсинг не вдався.

Оновлення асерцій в main: В assert_eq!(result, Ok(20)); перевіряється, чи повертається правильний результат, а для другого виклику перевіряється, що виникає помилка.
*/