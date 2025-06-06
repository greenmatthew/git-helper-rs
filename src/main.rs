use clap::{Arg, Command};
use std::process::exit;
mod commands;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LICENSE: &str = include_str!("../LICENSE");

fn main() {
    let matches = Command::new("git-helper")
        .version(VERSION)
        .about("A CLI tool to simplify Git repository management")
        .arg_required_else_help(true)
        .arg(
            Arg::new("license")
                .long("license")
                .help("Display the license information")
                .action(clap::ArgAction::SetTrue),
        )
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
        .subcommand(
            Command::new("remote")
                .about("Commands for managing Git remotes")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("reinit")
                        .about("Reinitializes the remote configuration for an existing repository")
                        .arg(
                            Arg::new("remote")
                                .long("remote")
                                .value_names(["NAME", "URL"])
                                .help("Add a remote to the repository. When multiple remotes are specified, an 'all' remote is automatically created. The first remote is used as the primary remote for fetching, and all remotes are added for pushing.")
                                .num_args(2)
                                .action(clap::ArgAction::Append)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("submodule")
                .about("Commands for managing Git submodules")
                .arg_required_else_help(true)  // Add this line
                .subcommand(
                    Command::new("purge")
                        .about("Completely removes a submodule by deinitializing, removing from git modules, and deleting the directory")
                        .arg(
                            Arg::new("PATH")
                                .help("Path to the submodule to purge")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
        .get_matches();

    if matches.get_flag("license") {
        println!("{LICENSE}");
        return;
    }

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
        Some(("remote", sub_matches)) => {
            if let Some(("reinit", reinit_matches)) = sub_matches.subcommand() {
                let remotes = if let Some(values) = reinit_matches.get_many::<String>("remote") {
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
                
                if let Err(e) = commands::remote::reinit::execute(&remotes) {
                    eprintln!("Error: {e}");
                    exit(1);
                }
            }
        }
        Some(("submodule", sub_matches)) => {
            if let Some(("purge", purge_matches)) = sub_matches.subcommand() {
                if let Some(path) = purge_matches.get_one::<String>("PATH") {
                    if let Err(e) = commands::submodule::purge(path) {
                        eprintln!("Error: {e}");
                        exit(1);
                    }
                }
            }
        }
        _ => {
            panic!("Unexpected command - This should not be reached due to arg_required_else_help(true)");
        }
    }
}
