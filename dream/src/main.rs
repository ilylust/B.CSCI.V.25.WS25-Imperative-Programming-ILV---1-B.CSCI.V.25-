// EX 2
// mutates a number by adding another number n.
fn mutate_number(number: &mut i32, number2: &i32) {
    *number += *number2
}
// concatenates two strings.
fn concat_strings(string1: &str, string2: &str) -> String {
    String::from(String::from(string1) + string2)
}
// swaps the values of two variables.
// since generics weren't covered yet we are using type i32
fn swap_two(variable1: &mut i32, variable2: &mut i32) {
    let temp = *variable1;
    *variable1 = *variable2;
    *variable2 = temp;
}
// returns a nicely formatted string congratulation for a given birthday
fn happy_birthday(name: &str, age: u8) -> String {
    format!("Happy birthday {name}! Congratulations on turning {age} this year!")
}
fn main() {
    // mutate_number unit tests
    let mut number1: i32 = 1;
    let number2: i32 = 2;
    println!("{}, {}", &number1, &number2);
    mutate_number(&mut number1, &number2);
    println!("{}, {}", &number1, &number2);
    // concat_strings unit tests
    let string1: &str = "Hello there, ";
    let string2: &str = "Jason";
    println!("{}", concat_strings(&string1, &string2));
    // swap_two unit tests
    let mut number_one: i32 = 54;
    let mut number_two: i32 = 62;
    println!("{}, {}", &number_one, &number_two);
    swap_two(&mut number_one, &mut number_two);
    println!("{}, {}", &number_one, &number_two);
    // happy_birthday unit tests
    println!("{}", happy_birthday("Jason", 54));
    println!("{}", happy_birthday("Mason", 24));
    println!("{}", happy_birthday("Ason", 34));
}