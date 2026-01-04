// EX 6
use std::fmt::Display;
// an Iterator that yields all prime numbers up to a given number n.
#[derive(Debug)]
struct PrimeNumbers { 
    number: u32,
    current: u32
}

impl Iterator for PrimeNumbers {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        while self.current <= self.number {
            let n = self.current;
            self.current += 1;
            
            if n < 2 {
                continue;
            }
            
            let mut is_prime = true;
            let mut i = 2;
            
            while i * i <= n {
                if n % i == 0 {
                    is_prime = false;
                    break;
                }
                i += 1;
            }
            
            if is_prime {
                return Some(n);
            }
        }
        None
    }
}
// NewType to implement maximum number n
struct PrimesUpTo(u32);
// Iterator for the newtype
impl IntoIterator for PrimesUpTo {
    type Item = u32;
    type IntoIter = PrimeNumbers;
    
    fn into_iter(self) -> Self::IntoIter {
        PrimeNumbers {
            number: self.0,
            current: 2
        }
    }
}
// types that represent an address book and implement Debug/Display
// store individual information
#[derive(Clone)]
struct Information<'a> {
    name: &'a str,
    phone: &'a str,
    email: &'a str,
    address: &'a str
}

impl<'a> Display for Information<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Name: {} \nPhone: {} \nEmail: {} \nAddress: {}", self.name, self.phone, self.email, self.address)
    }
}

struct AddressBook<'a>(Vec<Information<'a>>);

struct AddressBookIntoIter<'a>(&'a AddressBook<'a>);

impl<'a> IntoIterator for AddressBookIntoIter<'a> {
    type Item = &'a Information<'a>;
    type IntoIter = std::slice::Iter<'a, Information<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.0.iter()
    }
}

impl<'a> From<(&'a str, &'a str, &'a str, &'a str)> for Information<'a> {
    fn from(tuple: (&'a str, &'a str, &'a str, &'a str)) -> Self { 
        Information {
            name: tuple.0,
            phone: tuple.1,
            email: tuple.2,
            address: tuple.3
        }
    }
}

impl<'a> From<Information<'a>> for (&'a str, &'a str, &'a str, &'a str) {
    fn from(info: Information<'a>) -> Self { 
        (info.name, info.phone, info.email, info.address)
    }
}

impl<'a> TryFrom<&'a str> for Information<'a> {
    type Error = &'static str;
    fn try_from(information: &'a str) -> Result<Self, Self::Error> {
        // loop to extract chars and take out information from the string
        // divider will be ","
        if information.is_empty() {
            return Err("Input string is empty");
        }
        let mut parts = information.split(',').map(|s| s.trim());
        // extract the info
        Ok(Information {
            name: parts.next().ok_or("Missing name")?,
            phone: parts.next().ok_or("Missing phone")?,
            email: parts.next().ok_or("Missing email")?,
            address: parts.next().ok_or("Missing address")?
        })
    }
}

impl<'a> From<&Information<'a>> for String {
    fn from(info: &Information<'a>) -> String {
        format!("{},{},{},{}", info.name, info.phone, info.email, info.address)
    }
}

fn main() {
    // Prime
    let primes = PrimesUpTo(100);
    // Address
    let my_address = Information::from(("Tutu", "5839168", "myemail@gmail.com", "Ulica kriza"));
    let other_address = Information::from(("Bubu", "6136717", "myemail2@gmail.com", "Ulica kriza petrova"));
    
    let mut book = AddressBook(Vec::new());
    book.0.push(my_address.clone());
    book.0.push(other_address.clone());
    let address_collection = AddressBookIntoIter(&book);

    for info in address_collection {
        println!("{}", info.name);
    }

    // tuple test for address book
    let info_tuple: (&str, &str, &str, &str) = other_address.into();
    println!("{:?}", info_tuple);
    // tryfrom testing...
    let some_information = "Abacus,3681961386,painender@book.com,Saint st";
    let empty_string: &str = "";
    let testing_some_information = Information::try_from(some_information).unwrap();
    println!("Name: {}", testing_some_information.name);
    println!("Phone: {}", testing_some_information.phone);
    println!("Email: {}", testing_some_information.email);
    println!("Address: {}", testing_some_information.address);
    // try from implemented
    let abcd: String = String::try_from(&testing_some_information).unwrap();
    let abcde: String = (&testing_some_information).into();
    let empty_string: &str = "";
    let result = Information::try_from(empty_string);
    println!("{}", abcd);
    println!("{}", abcde);
    match result {
        Ok(x) => println!("Got a string {}", x),
        Err(e) => println!("Error: {}", e) 
    }
    // Prime printing
    println!("{:?}", primes.into_iter().collect::<Vec<u32>>());
}