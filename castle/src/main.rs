// EX 3
// finds the minimum and the sum of a collection of numbers.
fn min_and_sum(numbers: &[i32]) -> (i32, i32) {
    let mut min: i32 = i32::MAX;
    let mut sum: i32 = 0;
    if numbers.is_empty() {
        return (0, 0);
    }
    for number in numbers {
        sum += *number;
        if *number < min {
            min = *number;
        }
    }
    (min, sum)
}
// prints the body mass index category (2nd exercise) using pattern matching.
fn print_bmi(weight: f32, height: f32) {
    let bmi = weight / (height * height);
    match bmi {
        ..18.5 => println!("Underweight"),
        18.5..25.0 => println!("Healthy weight"),
        25.0..30.0 => println!("Overweight"),
        30.0.. => println!("Obesity"),
        _ => println!("Error")
    }
}
// returns the longest prefix of a collection having numbers smaller than n.
fn longest_prefix(numbers: &[i32], n: i32) -> Vec<i32> {
    let mut current_prefix: Vec<i32> = Vec::new();
    for number in numbers {
        if *number < n {
            current_prefix.push(*number);
        } else {
            break;
        }
    }
    current_prefix
}
// returns all prime numbers up to a given number n.
fn return_primes(n: u32) -> Vec<u32> {
    let mut count: u32;
    let mut starting_number: u32 = 2;
    let mut index: u32;
    let mut prime_numbers: Vec<u32> = Vec::new();
    while starting_number <= n {
        count = 0;
        index = 2;
        while index <= starting_number / 2 {
            if starting_number % index == 0 {
                count += 1;
                break;
            }
            index += 1;
        }
        if count == 0 {
            prime_numbers.push(starting_number);
        }
        starting_number += 1;
    }
    prime_numbers
}
// sorts a collection of people with names and ages by name
fn sort_collection_by_name<'a>(collection: &'a[(&'a str, u32)]) -> Vec<(&'a str, u32)> {
    let mut sorted_collection: Vec<(&str, u32)> = Vec::new();
    for tuples in collection {
        sorted_collection.push((tuples.0, tuples.1));
    }
    
    let len = sorted_collection.len();
    let mut i = 0;
    while i < len {
        let mut index = i;
        let mut j = i + 1;
        
        while j < len {
            if sorted_collection[j].0 < sorted_collection[index].0 {
                index = j;
            }
            j += 1;
        }
        
        if index != i {
            let temp = sorted_collection[i];
            sorted_collection[i] = sorted_collection[index];
            sorted_collection[index] = temp;
        }
        
        i += 1;
    }
    sorted_collection
}
/* 
    prints a collection of heroes with names and health points in a nice way by
    visualizing them below each other with their health points as health bars.
    Hero - █████░░
    Wizard - XXX
    Ranger - XXX
    Brute - XXX
*/
// &str Name, u8 HP
fn print_heroes(collection: &[(&str, u8)]) {
    let mut i: usize = 0;
    let len = collection.len();
    let mut current_hp: String;
    // total hp ░ ░ ░ ░ ░ ░ ░ ░ ░ ░ <- times
    // current  █ █ █ █ █ █ █ █ █ █
    while i < len {
        match collection[i].1 {
            0 => current_hp = String::from("▒▒▒▒▒▒▒▒▒▒"),
            1..10 => current_hp = String::from("▓▒▒▒▒▒▒▒▒▒"),
            10..20 => current_hp = String::from("▓▓▒▒▒▒▒▒▒▒"),
            20..30 => current_hp = String::from("▓▓▓▒▒▒▒▒▒▒"),
            30..40 => current_hp = String::from("▓▓▓▓▒▒▒▒▒▒"),
            40..50 => current_hp = String::from("▓▓▓▓▓▒▒▒▒▒"),
            50..60 => current_hp = String::from("▓▓▓▓▓▓▒▒▒▒"),
            60..70 => current_hp = String::from("▓▓▓▓▓▓▓▒▒▒"),
            70..80 => current_hp = String::from("▓▓▓▓▓▓▓▓▒▒"),
            80..90 => current_hp = String::from("▓▓▓▓▓▓▓▓▓▒"),
            90..=100 => current_hp = String::from("▓▓▓▓▓▓▓▓▓▓"),
            _ => current_hp = "?".to_string()
        }
        println!("{} \n{}", collection[i].0, current_hp);
        i += 1;
    }
}
fn main() {
    // min_and_sum unit tests
    let collection_i32: Vec<i32> = vec![1, 2, 3, 4, 5];
    let collection2_i32: Vec<i32> = vec![5, 4, 3, 2, 1];
    let collection3_i32: Vec<i32> = vec![-1, -2, -3, -4, -5];
    println!("{:?}", min_and_sum(&collection_i32));
    println!("{:?}", min_and_sum(&collection2_i32));
    println!("{:?}", min_and_sum(&collection3_i32));
    // print_bmi unit tests
    print_bmi(10.0, 1.92);
    print_bmi(92.0, 1.92);
    print_bmi(100.0, 1.92);
    print_bmi(2000.0, 1.92);
    // longest_prefix unit tests
    println!("{:?}", longest_prefix(&collection_i32, 5));
    println!("{:?}", longest_prefix(&collection_i32, 6));
    println!("{:?}", longest_prefix(&collection3_i32, -1));
    // return_primes unit tests
    println!("{:?}", return_primes(10));
    println!("{:?}", return_primes(50));
    println!("{:?}", return_primes(100));
    // sort_collection_by_name unit tests
    let collection1 = [("D", 5), ("B", 6), ("C", 7), ("A", 8)];
    println!("{:?}", sort_collection_by_name(&collection1));
    // print_heroes unit tests
    let collection_heroes = [("Hero", 10), ("Bard", 50), ("Ranger", 100), ("Thief", 99), ("Demon", 0)];
    print_heroes(&collection_heroes);
}