use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String
}

/*
1. Add Contact
2. Show Contact
3. Search Contacts
 */
fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        decorate();
        println!("Phone Book Application");
        decorate();

        println!("Please select an option");
        load_menus();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("error while making choice");

        match choice.trim() {
            "1" => add_contact(&mut contacts),
            "2" => show_contacts(&contacts),
            "3" => search_contacts(&mut contacts),
            "4" => {
                break;
            },
            _ => println!("Invalid choice"),
        }
    }
}

fn load_menus(){
    println!("1) Add Contact");
    println!("2) Show Contacts");
    println!("3) Search Contact");
    println!("4) Exit");
}

fn add_contact(contacts: &mut Vec<Contact>){
    decorate();
    let mut name = String::new();
    let mut phone = String::new();

    println!("Enter contact name");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter contact phone");
    io::stdin().read_line(&mut phone).unwrap();

    let contact = Contact {
        name: name.trim().to_string(),
        phone: phone.trim().to_string(),
    };

    contacts.push(contact);

    println!("Successfully created contact, current phone book size is {}", contacts.len());

    decorate();

}

fn show_contacts(contacts: &Vec<Contact>){
    decorate();
    println!("Phone Book List");
    for (i, contact) in contacts.iter().enumerate() {
        println!("{}) {} {}", i, contact.name, contact.phone);
    }
    decorate();
}

fn search_contacts(contacts: &mut Vec<Contact>){
    println!("Enter the name to search");

    let mut name_to_search = String::new();
    io::stdin().read_line(&mut name_to_search).unwrap();
    let name_to_search = name_to_search.trim().to_lowercase();

    let results: Vec<&Contact> = contacts
        .iter()
        .filter(|contact| contact.name.to_lowercase().contains(&name_to_search))
        .collect();

    if results.is_empty() {
        println!("No contacts found with name '{}'", name_to_search);
    } else {
        println!("Found {} contacts", results.len());
        for (i, contact) in results.iter().enumerate() {
            println!("{}) {} {}", i, contact.name, contact.phone);
        }
    }

    decorate();
}

fn decorate() {
    println!("========================================");
}
