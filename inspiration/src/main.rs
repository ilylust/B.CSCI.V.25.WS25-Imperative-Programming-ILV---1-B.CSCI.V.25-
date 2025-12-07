// EX 1
// calculates the absolute value of a number
fn absolute_value(number: i32) -> u32 {
    if number < 0 {
        return -number as u32;
    }
    number as u32
}
// enforces a number to be in a given range.
fn enforce_range(number: i32, minimum: i32, maximum: i32) -> i32 {
    if number >= minimum && number <= maximum {
        number
    } else if number < minimum {
        minimum
    } else {
        maximum
    }
}
// determines the maximum of three numbers.
fn maximum_of_three(n1: i32, n2: i32, n3: i32) -> i32 {
    if n1 > n2 && n1 > n3 {
        n1
    } else if n2 > n1 && n2 > n3 {
        n2
    } else {
        n3
    }
}
// determines the amount of days for a given month. (not including leap years)
fn days_in_month(month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 0
    }
}
// prints the grade for this course based on four test results.
// 12.5% per grade
// 87.5
// 75
// 62.5
// 50
fn print_grade(test1: f32, test2: f32, test3: f32, test4: f32) {
    let accumulated_grade: f32 = test1 + test2 + test3 + test4;
    match accumulated_grade {
        ..=50.0 => println!("Grade 5"),
        50.1..=62.5 => println!("Grade 4"),
        62.51..=75.0 => println!("Grade 3"),
        75.1..=87.5 => println!("Grade 2"),
        87.51.. => println!("Grade 1"),
        _ => println!("Error")
    }
}
// prints the body mass index category for a given weight and height.
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

fn main() {
    // absolute_value unit tests
    println!("{}", absolute_value(5));
    println!("{}", absolute_value(-5));
    println!("{}", absolute_value(-15));
    // enforce_range unit tests
    println!("{}", enforce_range(-15, -20, 50));
    println!("{}", enforce_range(5, 1, 50));
    println!("{}", enforce_range(62, 1, 50));
    println!("{}", enforce_range(-15, 1, 50));
    // maximum_of_three unit tests
    println!("{}", maximum_of_three(-15, -20, 50));
    println!("{}", maximum_of_three(5, 1, 24));
    println!("{}", maximum_of_three(62, 1, 0));
    println!("{}", maximum_of_three(-2664, 16, 462));
    // days_in_month unit tests
    println!("{}", days_in_month(1));
    println!("{}", days_in_month(2));
    println!("{}", days_in_month(3));
    println!("{}", days_in_month(4));
    println!("{}", days_in_month(5));
    println!("{}", days_in_month(6));
    println!("{}", days_in_month(7));
    println!("{}", days_in_month(8));
    println!("{}", days_in_month(9));
    println!("{}", days_in_month(10));
    println!("{}", days_in_month(11));
    println!("{}", days_in_month(12));
    // print_grade unit tests
    print_grade(100.0, 100.0, 100.0, 100.0);
    print_grade(50.0, 27.0, 0.0, 0.0);
    print_grade(13.5, 13.5, 20.0, 16.5);
    print_grade(25.0, 35.0, 0.0, 0.0);
    print_grade(0.0, 0.0, 0.0, 0.0);
    // print_bmi unit tests
    print_bmi(10.0, 1.92);
    print_bmi(92.0, 1.92);
    print_bmi(100.0, 1.92);
    print_bmi(2000.0, 1.92);
}