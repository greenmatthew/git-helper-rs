# git-helper-rs

A simple CLI tool that provides convenient abstractions for common Git commands. Built in Rust using the [Clap](https://github.com/clap-rs/clap) command line argument parser.

## Features

- Initialize Git repositories with multiple remotes at once
- Automatically configure an "all" remote for pushing to multiple remotes simultaneously
- Completely purge Git submodules with a single command

## Installation

There are two ways to install `git-helper`:

<details>
<summary><b>User-level installation</b></summary>

Install the binary to your Cargo bin directory:

```bash
just install
```

Note: Make sure Cargo's bin directory is part of your PATH, you'll need to do that to run git-helper from anywhere.

</details>
<details>
<summary><b>System-wide installation</b></summary>

Install the binary to /usr/local/bin (requires sudo):

```bash
just install-system
```

This method makes the binary available to all users on the system.

</details>

## Usage

For a list of available commands and options:

```
git-helper help
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