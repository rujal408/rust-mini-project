mod enums;
use enums::Command;
mod utils;
use colored::*;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use utils::parse_command;

struct Todo {
    text: String,
    completed: bool,
}

fn get_todos(file_name: &str) -> Vec<Todo> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("❌ Failed to open file");
            return Vec::new();
        }
    };

    let content = BufReader::new(file);

    let mut todos: Vec<Todo> = Vec::new();

    for (_, line_result) in content.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };
        let completed = line.starts_with("[x]");
        let text = line[4..].to_string();

        todos.push(Todo { text, completed });
    }
    todos
}

fn update_file(todos: Vec<Todo>, file_name: &str) {
    let mut file = fs::File::create(file_name).expect("Failed to open file");
    for todo in &todos {
        let status = if todo.completed { "[x]" } else { "[ ]" };
        writeln!(file, "{} {}", status, todo.text).expect("Failed to write");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (command, file_name) = match parse_command(args) {
        Some(res) => res,
        None => return,
    };

    match command {
        Command::Create => {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_name)
                .expect("Failed to create file");

            println!("File {} has been created", file_name);
        }
        Command::Add(value) => {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_name)
                .expect("Failed to open file");

            writeln!(file, "[ ] {}", value).expect("Failed to write");
            println!("Task added: {}", value);
        }

        Command::List => {
            let todos = get_todos(&file_name);

            for todo in &todos {
                let status = if todo.completed { "[x]" } else { "[ ]" };
                println!("{} {}", status, todo.text);
            }
        }

        Command::Edit(index, text) => {
            let mut todos = get_todos(&file_name);
            todos[index].text = text;
            update_file(todos, &file_name);
        }

        Command::Delete(index) => {
            let mut todos = get_todos(&file_name);
            todos.remove(index);
            update_file(todos, &file_name);
        }

        Command::Complete(index) => {
            let mut todos = get_todos(&file_name);
            todos[index].completed = true;
            update_file(todos, &file_name);
        }

        Command::Clear => {
            println!("Are you sure? (y/n)");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");

            if input.trim() == "y" {
                fs::write(&file_name, "").expect("Failed to clear file");
                println!("All tasks cleared");
            } else {
                println!("Cancelled");
            }
        }

        Command::Help => {
            println!("Available commands:");
            println!("add filename <task>                                          Add a new task");
            println!("list filename                                                Show all tasks");
            println!(
                "complete filename <index>                                    Mark task as completed"
            );
            println!("delete filename <index>                                      Delete a task");
            println!("edit filename <index> <text>                                 Edit a task");
            println!(
                "clear filename                                               Remove all tasks"
            );
            println!("help                                                Show this help message");
            println!("grep -i(optional) -n(optional) <keyword> <filename> Search content from file")
        }

        Command::Grep {
            keyword,
            ignore_case,
            show_line_number,
        } => {
            println!("Keyword: {}", keyword);
            println!("Ignore case: {}", ignore_case);
            println!("Show line number: {}", show_line_number);
            println!("File name: {}", file_name);
            let file = match File::open(&file_name) {
                Ok(f) => f,
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

            let content = BufReader::new(file);

            for (i, line_result) in content.lines().enumerate() {
                let line = match line_result {
                    Ok(l) => l,
                    Err(_) => continue,
                };

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
