# Rust: Todo CLI

Todo App for CLI, make with Rust

## Features

- Can list, add, remove, toggle, and clear todos
- Store todos in a json file
- Set custom file to store todos

## Build

```bash
# Build for debug purpose
cargo build

# Build for release purpose
cargo build --release
```

## Run

```bash
# Build and run for debug purpose
cargo run -- --help

# Run after release build
./target/release/demo-rust-cli --help
```

## Usage

```bash
# Show todo list, optional list only todos with status
demo-rust-cli show  [--only=completed|pending]

# Add a todo
demo-rust-cli add 'some todo description'

# Remove a todo by ID
demo-rust-cli remove todo-id

# Toggle todo status by ID
demo-rust-cli toggle todo-id

# Clear all todos, optional clear only todos with status
demo-rust-cli clear  [--only=completed|pending]

# All commands support set a custom db file, by default use db.json
demo-rust-cli --db custom-db.json show
```