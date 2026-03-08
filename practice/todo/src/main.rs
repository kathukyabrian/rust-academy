use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        show_main_menu();
        let mut choice = String::new();
        println!("Select an action to take...");

        // get user input
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to get user input");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => show_tasks(&tasks),
            "3" => {
                exit();
                break;
            },
            _ => println!("Invalid choice, please try again"),
        }
    }
}

fn show_main_menu(){
    println!("Welcome to Task Manager App");
    println!("Select an option:");
    println!("1) Add task");
    println!("2) Show tasks");
    println!("3) Exit");
}

/*
   in this method, we expect to get the vector, get user input for task and add it to the vector should be mutable
 */
fn add_task(tasks: &mut Vec<String>) {
    decorate();
    println!("Add task:");

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    tasks.push(task.trim().to_string());
    println!("Added task: '{}' successfully", task.trim());
    decorate();
}

fn show_tasks(tasks: &Vec<String>) {
    decorate();
    println!("Task List:");
    decorate();
    for (i,task) in tasks.iter().enumerate() {
        println!("{}){}", i + 1, task);
    }
    decorate();
}

fn decorate(){
    println!("===================================");
}

fn exit(){

}

