mod enums;
use enums::Command;
mod utils;
use colored::*;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use utils::parse_command;

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

        Command::Clear => {
            println!("Are you sure? (y/n)");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");

            if input.trim() == "y" {
                fs::write("todos.txt", "").expect("Failed to clear file");
                println!("All tasks cleared");
            } else {
                println!("Cancelled");
            }
        }

        Command::Help => {
            println!("Available commands:");
            println!("add <task>                Add a new task");
            println!("list                      Show all tasks");
            println!("complete <index>          Mark task as completed");
            println!("delete <index>            Delete a task");
            println!("edit <index> <text>       Edit a task");
            println!("clear                     Remove all tasks");
            println!("help                       Show this help message");
            println!("grep <keyword> <filename> Search content from file")
        }

        Command::Grep {
            keyword,
            filename,
            ignore_case,
            show_line_number,
        } => {
            let content = match fs::read_to_string(&filename) {
                Ok(data) => data,
                Err(_) => {
                    println!("❌ Failed to read file");
                    return;
                }
            };

            let keyword_lower = if ignore_case {
                keyword.to_lowercase()
            } else {
                keyword.clone()
            };

            for (i, line) in content.lines().enumerate() {
                let line_lower = line.to_lowercase();
                if line_lower.contains(&keyword_lower) {
                    let mut highlighted = String::new();

                    let mut start = 0;

                    while let Some(pos) = line_lower[start..].find(&keyword_lower) {
                        let real_pos = start + pos;

                        // push text before match
                        highlighted.push_str(&line[start..real_pos]);

                        // push highlighted match
                        let end = real_pos + keyword.len();
                        highlighted.push_str(&format!(
                            "{}",
                            &line[real_pos..end].red().bold().to_string()
                        ));

                        start = end;
                    }

                    // push remaining text
                    highlighted.push_str(&line[start..]);

                    if show_line_number {
                        println!("{}: {}", i + 1, highlighted);
                    } else {
                        println!("{}", highlighted);
                    }
                }
            }
        }
    }
}
