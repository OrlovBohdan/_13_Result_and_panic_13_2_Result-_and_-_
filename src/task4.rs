#[test]

/*
use std::num::ParseIntError;

// FILL in the blank in two ways: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().__
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}
*/
fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}
use std::num::ParseIntError;

// FILL in the blank in two ways: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}




/*
map: Якщо парсинг успішний, виконує лямбда-функцію, що додає 2 до отриманого числа.
and_then: Якщо парсинг успішний, повертає новий Result за допомогою Ok(n + 2).
*/