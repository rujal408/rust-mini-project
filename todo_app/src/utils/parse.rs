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

        _ => {
            println!("Unknown command");
            None
        }
    }
}
