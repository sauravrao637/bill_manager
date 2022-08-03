//   Interactive bill manager
//   A command line bills manager that runs
//   interactively.
//
//   Features
//   - Add bills, including the name and amount owed.
//   - View existing bills.
//   - Remove bills.
//   - Edit existing bills.
//   - Go back if user change my mind.

use std::io;
use std::collections::HashMap;

fn get_input() -> Option<String> {
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Error reading input");
    }
    let input = buf.trim().to_owned();
    if input.is_empty() {
        return None;
    }
    return Some(input);
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill,
    EditBill,
    Exit,
}

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f32,
}


impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBills),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::EditBill),
            "5" => Some(MainMenu::Exit),
            _ => None
        }
    }

    fn show(total: usize) {
        println!("==============================");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");
        println!("Total: {}", total);
        println!("Enter your choice: ");
        println!("==============================");
    }
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();
    loop {
        MainMenu::show(bills.len());
        let input = get_input().expect("Error reading input");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => {
                println!("Enter the name of the bill: ");
                let name = get_input().expect("Error reading input");
                println!("Enter the amount of the bill: ");
                let amount = get_input().expect("Error reading input").parse::<f32>().expect("Error parsing input");
                bills.insert(name.to_owned(), Bill { name: name, amount: amount });
            }
            Some(MainMenu::ViewBills) => {
                for bill in bills.values() {
                    println!("{:?}", bill);
                }
            }
            Some(MainMenu::RemoveBill) => {
                println!("Enter the name of the bill: ");
                let name = get_input().expect("Error reading input");
                if bills.remove(&name).is_none() {
                    println!("Bill not found");
                }
            }
            Some(MainMenu::EditBill) => {
                println!("Enter the name of the bill: ");
                let name = get_input().expect("Error reading input");
                let bill = bills.get_mut(&name);
                if bill.is_none() {
                    println!("Bill not found");
                    continue;
                }
                println!("Enter the new amount of the bill: ");
                let amount = get_input().expect("Error reading input").parse::<f32>().expect("Error parsing input");
                println!("Enter Ok to confirm or Cancel to go back: ");
                let input = get_input().expect("Error reading input");
                match input.as_str() {
                    "Ok" => {
                        bill.unwrap().amount = amount;
                    }
                    "Cancel" => {
                        println!("Going back");
                    }
                    _ => {
                        println!("Invalid input");
                    }
                }
            }
            Some(MainMenu::Exit) => {
                println!("Exit");
                break;
            }
            None => {
                println!("Invalid choice");
            }
        }
    }
}
