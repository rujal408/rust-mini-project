use crate::enums::Command;

pub fn parse_command(args: Vec<String>) -> Option<Command> {
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
            Some(Command::Add(args[2..].join(" ").clone()))
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
            Some(Command::Edit(index, args[3..].join(" ").clone()))
        }

        "clear" => Some(Command::Clear),
        "help" => Some(Command::Help),
        "grep" => {
            if args.len() < 4 {
                println!("Usage: grep [-i] [-n] <keyword> <file>");
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
            let filename = args[index + 1].clone();

            Some(Command::Grep {
                keyword,
                filename,
                ignore_case,
                show_line_number,
            })
        }

        _ => {
            println!("Unknown command");
            None
        }
    }
}
