mod commands {
    pub mod init;
    pub mod hash_object;
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("Usage: {} help", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "init" => commands::init::initialize_repo(),
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
