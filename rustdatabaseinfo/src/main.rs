use postgres::Client;
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;
use std::error::Error;
use std::io::{self,Write};

fn string_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut response : String = String::new();
    io::stdin().read_line(&mut response).unwrap();
    return response.trim().to_string();
}

fn new_supplier(client: &mut postgres::Client) {
    let supplier_name = string_input("Input supplier name");
    let contact_name = string_input("Input contact name");
    let phone = string_input("Input phone number");
    let email = string_input("Input email address");
    let address = string_input("Input address");
    let city = string_input("Input city");
    let state = string_input("Input state (2 letters)");
    let zip_code = string_input("Input zip code");

    let query = "INSERT INTO suppliers(supplier_name, contact_name, phone, email, address, city, state, zip_code) VALUES($1, $2, $3, $4, $5, $6, $7, $8) RETURNING supplier_id;";

    match client.query_one(query, &[&supplier_name, &contact_name, &phone, &email, &address, &city, &state, &zip_code]) {
        Ok(row) => {
            let supplier_id: i32 = row.get(0);
            println!("New supplier with ID {}", supplier_id);
        }
        Err(e) => {
            println!("Error inserting supplier: {}", e);
        }
    }
}

fn print_supplier_purchase_summary(client: &mut postgres::Client) { 
    let rows = client.query("SELECT supplier_name, number_of_orders, total_purchase_amount::TEXT FROM v_supplier_purchase_summary;", &[]).unwrap();
    println!("| {:35} | {:35} | {:35} |", "Supplier name", "Number of orders", "Total purchase amount");
    println!("| {:35} | {:35} | {:35} |", "","","");
    for row in rows {
        let supplier_name: String = row.get("supplier_name");
        let order_count: i64 = row.get("number_of_orders");
        let purchase_amount: Option<String> = row.get("total_purchase_amount");
        println!("| {:35} | {:35} | {:35} |", supplier_name, order_count, purchase_amount.unwrap());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let connector = TlsConnector::new()?;
    let connector = MakeTlsConnector::new(connector);
    let database_url = "postgres://CONNECTION_STR";
    let mut client = Client::connect(&database_url, connector)?;

    new_supplier(&mut client);
    print_supplier_purchase_summary(&mut client);

    client.close()?;
    io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}