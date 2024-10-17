#[test]

/*
use std::num::ParseIntError;

// FILL in the blank
type __;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}
*/
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}

use std::num::ParseIntError;

// FILL in the blank
type Res<T> = Result<T, ParseIntError>; // Створюємо типове скорочення для Result

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}



/*
type Res<T> = Result<T, ParseIntError>;: Це визначення типу створює псевдонім Res, який представляє Result тип
з успішним значенням T та помилкою типу ParseIntError. Тепер ви можете використовувати Res<i32> замість Result<i32,
ParseIntError> у всьому коді.
У функціях multiply і print ви тепер використовуєте Res<i32>, що робить код більш чистим і легшим для читання.
*/