mod commands {
    pub mod init;
    pub mod hash_object;
    pub mod cat_file;
    pub mod write_tree;
    pub mod commit_tree;
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
            commands::hash_object::hash_object(file_path, write, true);
        }
        "cat-file" => {
            if args.len() < 3 {
                eprintln!("Usage: {} cat-file [-p] <hash>", args[0]);
                std::process::exit(1);
            }
            let print = args[2] == "-p";
            let hash = if print { &args[3] } else { &args[2] };
            commands::cat_file::cat_file(hash, print);
            
        }
        "write-tree" => {
            if args.len() != 2 {
                eprintln!("Usage: {} write-tree", args[0]);
                std::process::exit(1);
            }
            commands::write_tree::write_tree();
        }
        "commit-tree" => {
            if args.len() != 3 && args.len() != 5 {
                eprintln!("Usage: {} commit-tree <hash> [-m <message>]", args[0]);
                std::process::exit(1);
            }
            let hash = &args[2];
            let message = if args.len() > 3 && args[3] == "-m" {
                Some(args[4].clone())
            } else {
                None
            };
            commands::commit_tree::commit_tree(hash, message.as_deref().unwrap_or("commit!"));
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
