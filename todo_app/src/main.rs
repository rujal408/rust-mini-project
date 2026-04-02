use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

struct Todo {
    text: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let todos: Vec<Todo> = vec![];

    if args.len() < 2 {
        println!("Usage: cargo run <command> <value>");
        return;
    }
    let command = args[1].as_str();

    match command {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add");
                return;
            }
            let value = &args[2];

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("todos.txt")
                .expect("Failed to open file");

            writeln!(file, "[ ] {}", value).expect("Failed to write");
            println!("Task added: {}", value);
        }
        "list" => {
            let content = fs::read_to_string("todos.txt").unwrap_or(String::from("No tasks found"));
            let mut todos: Vec<Todo> = Vec::new();

            for line in content.lines() {
                let completed = line.starts_with("[x]");
                let text = line[4..].to_string();

                todos.push(Todo { text, completed });
            }

            for todo in &todos {
                let status = if todo.completed { "[x]" } else { "[ ]" };
                println!("{} {}", status, todo.text);
            }
        }

        "delete" => {}
        _ => println!("Unknown command"),
    }
}
