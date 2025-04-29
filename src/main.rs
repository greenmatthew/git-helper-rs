use clap::{Arg, Command};
use std::process::exit;
mod commands;

const VERSION: &str = "0.1.0";

fn main() {
    let matches = Command::new("git-helper")
        .version(VERSION)
        .about("A CLI tool to simplify Git repository management")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initializes a git repository")
                .arg(
                    Arg::new("remote")
                        .long("remote")
                        .value_names(["NAME", "URL"])
                        .help("Add a remote to the repository. When multiple remotes are specified, an 'all' remote is automatically created. The first remote is used as the primary remote for fetching, and all remotes are added for pushing.")
                        .num_args(2)
                        .action(clap::ArgAction::Append),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let remotes = if let Some(values) = sub_matches.get_many::<String>("remote") {
                let values: Vec<&String> = values.collect();
                let mut remotes = Vec::new();
                
                // Collect name-url pairs from the flattened values
                for chunk in values.chunks(2) {
                    if chunk.len() == 2 {
                        remotes.push((chunk[0].clone(), chunk[1].clone()));
                    }
                }
                remotes
            } else {
                Vec::new()
            };
            
            if let Err(e) = commands::init::execute(&remotes) {
                eprintln!("Error: {e}");
                exit(1);
            }
        }
        Some(("submodule", _sub_matches)) => {
            todo!("Implement git submodule commands.");
        }
        _ => {
        }
    }
}