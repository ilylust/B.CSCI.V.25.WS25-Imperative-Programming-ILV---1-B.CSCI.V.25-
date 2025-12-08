// EX 5
#[derive(Debug)]
enum Error {
    NoNumbers,
    EmptyList,
    NotFound
}
// finds the minimum of two values in a tuple
fn find_min<T: PartialOrd>(tuple: (T, T)) -> Result<T, Error> {
    if tuple.0 > tuple.1 {
        Ok(tuple.1)
    } else {
        Ok(tuple.0)
    }
}
// sums up a collection of optional numeric values.
fn sum_optional(values: &[Option<i32>]) -> Result<i32, Error> {
    let mut sum = 0;
    let mut found_any = false;
    for value in values {
        match value {
            Some(number) => {
                sum += * number;
                found_any = true;
            }
            None => {}
        }
    }
    
    if found_any {
        Ok(sum)
    } else {
        Err(Error::NoNumbers)
    }
}
// finds the maximum of a collection of elements having any type.
fn find_max<T: PartialOrd + Clone>(list: &[T]) -> Result<T, Error> {
    if list.is_empty() {
        return Err(Error::EmptyList);
    }
    // select first element as maximum
    let mut maximum = &list[0];
    
    for item in list {
        if item > maximum {
            maximum = item;
        }
    }
    
    Ok(maximum.clone())
}
// finds an element in a collection of any type and returns its index.
fn find_element<T: PartialEq>(list: &[T], element: &T) -> Result<usize, Error> {
    if list.is_empty() {
        return Err(Error::EmptyList);
    }
    let mut index: usize = 0;
    for item in list {
        if item == element {
            return Ok(index);
        }
        index += 1;
    }
    Err(Error::NotFound)
}
// sorts a collection of elements having any type (swap method is allowed).
fn sort_collection<T: PartialOrd + Clone>(collection: &[T]) -> Result<Vec<T>, Error> {
    if collection.is_empty() {
        return Err(Error::EmptyList);
    }
    // Create new collection so we dont modify the original
    let mut return_collection: Vec<T> = Vec::new();
    // Add all items to our new collection
    for item in collection {
        return_collection.push(item.clone());
    }
    
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..return_collection.len() - 1 {
            if return_collection[i] > return_collection[i + 1] {
                return_collection.swap(i, i + 1);
                swapped = true;
            }
        }
    }
    
    Ok(return_collection)
}
// joins elements of any type into a string using a given delimiter.
fn join_elements<T: std::fmt::Display, U: std::fmt::Display>(elements: &[T], delimiter: &U) -> Result<String, Error> {
    if elements.is_empty() {
        return Err(Error::EmptyList);
    }
    
    let mut result_string: String = String::new();
    // handle first without delimiter
    let mut first = true;
    for element in elements {
        if !first {
            result_string.push_str(&format!("{}", delimiter));
        }
        first = false;
        result_string.push_str(&format!("{}", element));
    }
    
    Ok(result_string)
}

fn main() {
    // find_min from tuple
    let tuple1 = (1, 2);
    let tuple2 = (1, 1);
    let tuple3 = ("a", "b");
    println!("{:?}", find_min(tuple1));
    println!("{:?}", find_min(tuple2));
    println!("{:?}", find_min(tuple3));
    // sum optional numerical values
    let values = [Some(5), None, Some(3), None, Some(10)];
    let values2 = [None, None, None, None];
    match sum_optional(&values) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:?}", e)
    }
    match sum_optional(&values2) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:?}", e)
    }
    // find maximum of collection (any type)
    let collection_str = ["c", "b", "a"];
    let collection_empty: Vec<i32> = vec![];
    let collection_i32 = [15, -15, 25];
    match find_max(&collection_str) {
        Ok(x) => println!("Maximum = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match find_max(&collection_empty) {
        Ok(x) => println!("Maximum = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match find_max(&collection_i32) {
        Ok(x) => println!("Maximum = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    // find element in collection (any type)
    match find_element(&collection_str, &"c") {
        Ok(x) => println!("Index = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match find_element(&collection_empty, &2) {
        Ok(x) => println!("Index = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match find_element(&collection_i32, &5) {
        Ok(x) => println!("Index = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    // join element of any type into a string using delimiter
    match join_elements(&collection_i32, &5) {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match join_elements(&collection_empty, &"|") {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match join_elements(&collection_i32, &"|") {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    // sort collection of any type (arr.swap)
    match sort_collection(&collection_i32) {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match sort_collection(&collection_empty) {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
    match sort_collection(&collection_str) {
        Ok(x) => println!("String = {:?}", x),
        Err(e) => println!("{:?}", e)
    }
}