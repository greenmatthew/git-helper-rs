# git-helper-rs

A simple CLI tool that provides convenient abstractions for common Git commands. Built in Rust using the [Clap](https://github.com/clap-rs/clap) command line argument parser.

## Features

- Initialize Git repositories with multiple remotes at once
- Automatically configure an "all" remote for pushing to multiple remotes simultaneously
- Completely purge Git submodules with a single command

## Usage

For a list of available commands and options:

```
git-helper --help
git-helper -h
```

## Examples

Initialize a repository with multiple remotes:
```
git-helper init --remote origin https://github.com/username/repo.git --remote gitlab https://gitlab.com/username/repo.git
```

Completely remove a submodule:
```
git-helper submodule purge path/to/submodule
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.