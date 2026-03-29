use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <command> <value>");
        return;
    }
    let command = &args[1];
    let value = &args[2];

    println!("Command {}", command);
    println!("Value {}", value);
}
