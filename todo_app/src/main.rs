use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

enum Command {
    Add(String),
    List,
    Complete(usize),
    Edit(usize, String),
    Delete(usize),
}

struct Todo {
    text: String,
    completed: bool,
}

fn get_todos() -> Vec<Todo> {
    let content = fs::read_to_string("todos.txt").unwrap_or(String::from("No tasks found"));
    let mut todos: Vec<Todo> = Vec::new();

    for line in content.lines() {
        let completed = line.starts_with("[x]");
        let text = line[4..].to_string();

        todos.push(Todo { text, completed });
    }
    todos
}

fn update_file(todos: Vec<Todo>) {
    let mut file = fs::File::create("todos.txt").expect("Failed to open file");
    for todo in &todos {
        let status = if todo.completed { "[x]" } else { "[ ]" };
        writeln!(file, "{} {}", status, todo.text).expect("Failed to write");
    }
}

fn parse_command(args: Vec<String>) -> Option<Command> {
    if args.len() < 2 {
        println!("Usage: cargo run <command> [value]");
        return None;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task");
                return None;
            }
            Some(Command::Add(args[2].clone()))
        }

        "list" => Some(Command::List),

        "complete" => {
            if args.len() < 3 {
                println!("Please provide index");
                return None;
            }

            match args[2].parse() {
                Ok(i) => Some(Command::Complete(i)),
                Err(_) => {
                    println!("Invalid number");
                    None
                }
            }
        }

        "delete" => {
            if args.len() < 3 {
                println!("Please provide index");
                return None;
            }

            match args[2].parse() {
                Ok(i) => Some(Command::Delete(i)),
                Err(_) => {
                    println!("Invalid number");
                    None
                }
            }
        }

        "edit" => {
            if args.len() < 4 {
                println!("Please provide <index> and <new text>");
                return None;
            }

            let index = match args[2].parse() {
                Ok(i) => i,
                Err(_) => {
                    println!("Invalid index");
                    return None;
                }
            };
            Some(Command::Edit(index, args[3].clone()))
        }

        _ => {
            println!("Unknown command");
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let todos: Vec<Todo> = vec![];

    if args.len() < 2 {
        println!("Usage: cargo run <command> <value>");
        return;
    }

    let command = match parse_command(args) {
        Some(cmd) => cmd,
        None => return,
    };

    match command {
        Command::Add(value) => {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("todos.txt")
                .expect("Failed to open file");

            writeln!(file, "[ ] {}", value).expect("Failed to write");
            println!("Task added: {}", value);
        }
        Command::List => {
            let todos = get_todos();

            for todo in &todos {
                let status = if todo.completed { "[x]" } else { "[ ]" };
                println!("{} {}", status, todo.text);
            }
        }

        Command::Edit(index, text) => {
            let mut todos = get_todos();
            todos[index].text = text;
            update_file(todos);
        }

        Command::Delete(index) => {
            let mut todos = get_todos();
            todos.remove(index);
            update_file(todos);
        }
        Command::Complete(index) => {
            let mut todos = get_todos();
            todos[index].completed = true;
            update_file(todos);
        }
    }
}
