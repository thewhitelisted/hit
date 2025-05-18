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
        "hash-object" => {
            if args.len() < 3 {
                eprintln!("Usage: {} hash-object [-w] <file>", args[0]);
                std::process::exit(1);
            }
            let write = args[2] == "-w";
            let file_path = if write { &args[3] } else { &args[2] };
            commands::hash_object::hash_object(file_path, write);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
