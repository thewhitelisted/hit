// the long list of death
mod commands {
    pub mod branch;
    pub mod cat_file;
    pub mod checkout;
    pub mod commit_tree;
    pub mod init;
    pub mod status;
    pub mod write_tree;
    pub mod add;
    pub mod commit;
}

// the marginally smaller list of death
pub mod utils {
    pub mod hash_object;
    pub mod objects;
    pub mod index;
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
            utils::hash_object::hash_object(file_path, write, true);
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
        "checkout" => {
            if args.len() != 3 {
                eprintln!("Usage: {} checkout <hash>", args[0]);
                std::process::exit(1);
            }
            let hash = &args[2];
            commands::checkout::checkout(hash);
        }
        "branch" => {
            let branch_name = if args.len() > 2 {
                Some(&args[2][..])
            } else {
                None
            };
            if let Err(e) = commands::branch::branch(branch_name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        "status" => {
            if args.len() != 2 {
                eprintln!("Usage: {} status", args[0]);
                std::process::exit(1);
            }
            commands::status::status();
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: {} add <file|directory>", args[0]);
                std::process::exit(1);
            }
            let path = &args[2];
            commands::add::add(path);
        }
        "commit" => {
            // if no message, return error
            if args.len() < 3 {
                eprintln!("Usage: {} commit -m <message>", args[0]);
                std::process::exit(1);
            }
            let message = if args[2] == "-m" {
                if args.len() < 4 {
                    eprintln!("Usage: {} commit -m <message>", args[0]);
                    std::process::exit(1);
                }
                Some(args[3].clone())
            } else {
                None
            };
            if let Some(msg) = message {
                commands::commit::commit(&msg);
            } else {
                eprintln!("Usage: {} commit -m <message>", args[0]);
                std::process::exit(1);
            }
        }
        // TODO: HELP!!!
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
