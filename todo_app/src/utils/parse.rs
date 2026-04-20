use crate::enums::Command;

pub fn parse_command(args: Vec<String>) -> Option<(Command, String)> {
    if args.len() < 3 {
        println!("Usage: cargo run <file_path> <command> [value]");
        println!("Example: cargo run -- files/todos.txt add \"Buy milk\"");
        return None;
    }

    let file_name = args[1].clone();
    println!("Overall args: {:?}", args);

    match args[2].as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Please provide a task");
                return None;
            }
            Some((Command::Add(args[3..].join(" ").clone()), file_name))
        }

        "list" => Some((Command::List, file_name)),

        "complete" => {
            if args.len() < 4 {
                println!("Please provide index");
                return None;
            }

            match args[3].parse() {
                Ok(i) => Some((Command::Complete(i), file_name)),
                Err(_) => {
                    println!("Invalid number");
                    None
                }
            }
        }

        "delete" => {
            if args.len() < 4 {
                println!("Please provide index");
                return None;
            }

            match args[3].parse() {
                Ok(i) => Some((Command::Delete(i), file_name)),
                Err(_) => {
                    println!("Invalid number");
                    None
                }
            }
        }

        "edit" => {
            if args.len() < 5 {
                println!("Please provide <index> and <new text>");
                return None;
            }

            let index = match args[3].parse() {
                Ok(i) => i,
                Err(_) => {
                    println!("Invalid index");
                    return None;
                }
            };
            Some((Command::Edit(index, args[4..].join(" ").clone()), file_name))
        }

        "clear" => Some((Command::Clear, file_name)),
        "help" => Some((Command::Help, file_name)),
        "grep" => {
            if args.len() < 5 {
                println!("Usage: cargo run <file_path> grep [-i] [-n] <keyword> <grep_file>");
                return None;
            }

            let mut ignore_case = false;
            let mut show_line_number = false;

            let mut index = 2;

            // Parse flags
            while args[index].starts_with("-") {
                match args[index].as_str() {
                    "-i" => ignore_case = true,
                    "-n" => show_line_number = true,
                    _ => {
                        println!("Unknown flag: {}", args[index]);
                        return None;
                    }
                }
                index += 1;
            }

            if args.len() <= index + 1 {
                println!("Missing keyword or filename");
                return None;
            }

            let keyword = args[index].clone();

            Some((
                Command::Grep {
                    keyword,
                    ignore_case,
                    show_line_number,
                },
                file_name,
            ))
        }

        _ => {
            println!("Unknown command");
            None
        }
    }
}
