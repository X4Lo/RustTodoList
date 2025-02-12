# Rust Todo List

A simple Rust application to manage your tasks with both interactive and command-line options.

---

## Features
- Add, update, remove, and view tasks.
- Interactive menu for manual task management.
- Command-line arguments for quick operations:
  - `--list`: View all tasks.
  - `--add "task title"`: Add a new task.
  - `--remove <id>`: Remove a task by ID.
  - `--update <id> <status>`: Update a task's status (e.g., `doing`, `done`).

## Installation
1. Ensure you have Rust installed: [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository or download the source code.
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Interactive Mode
Run the application in interactive mode:
```bash
./target/release/app
```
Follow the on-screen menu to manage your tasks.

### Command-Line Options  
Use the following commands for quick operations:  

```bash
# View Tasks
rust_todo_list.exe --list

# Add a Task
rust_todo_list.exe --add "New Task Title"

# Remove a Task
rust_todo_list.exe --remove 11

# Update a Task
rust_todo_list.exe --update 11 doing
```

> Replace `11` with the actual task ID and `doing` with the desired status (`todo`, `doing`, or `done`).

## File Structure
- **`main.rs`**: Application logic.
- **`tasks.txt`**: Persistent storage for tasks.

## Notes
- Ensure `tasks.txt` exists in the same directory as the binary.
- Status values are case-insensitive (e.g., `TODO`, `todo`, `ToDo` are all valid).
