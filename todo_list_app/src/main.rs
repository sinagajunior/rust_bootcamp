use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}



fn main() {
    let mut tasks: vec<Task> = load_task();
    
    loop {
     println!("\n To-Do List Menu:");
     println!("1. Add Task");
     println!("2. View task");
     println!("3. Mark task as Complete");
     println!("4. Delete Task");
     println!("5. Exit");

    let choice = get_input("Enter your choice: ");
    match choice.trim() {
       "1" => add_task(&mut tasks),
       "2" => view_task(&tasks),
       "3" => mark_task_complete(&mut tasks),
       "4" => delete_task(&mut tasks),
       "5" => {
           save_tasks(&tasks);
            println!("Tasks saved. Goodbye!"); 
            break;          
       }

    } 
    }


}

fn get_input(prompt: &str) -> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
input
}


// load tasks from file
fn load_task() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content)=> serde_json::from_str(&content).unwrap_or_else(|_|Vec::new()),
        Err(_) => Vec::new(),
    }
}

// Save tasks to file
fn save_tasks(tasks: &mut Vec<Task>){
    let description = get_input("Enter task description ");
    let id = tasks.len()+1;
    tasks.push(Task { id, 
        description:description.trim().to_string(),
         completed:false });
} 

// View all tasks
fn view_tasks(tasks: &Vec<Task>) {
    
}
