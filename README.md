# cli-todo-list-rs

# Rust CLI Todolist

Rust CLI Todolist is a command-line todo list manager built with Rust. It allows you to create, manage, and track tasks from the command line.

## Features

- Add new tasks
- Update task status, description, and name
- View tasks based on their status (done, todo, in progress, canceled)
- Print tasks in a nice table format
- Support for JSON file storage
- Change the current file name

## Dependencies

- [clap](https://crates.io/crates/clap): A powerful and easy-to-use command-line argument parsing library for Rust.
- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json): Libraries for serializing and deserializing Rust data structures to and from JSON.
- [chrono](https://crates.io/crates/chrono): A library for working with dates and times in Rust.
- [comfy-table](https://crates.io/crates/comfy-table): A crate for printing tabular data in a pleasant and comfortable way.

## Usage

To use the command-line todo list manager, you can run the following commands:

### Add a new task
cli_todo_list -a "New task name"
This command adds a new task with the specified name to the todo list.

### Update task status
cli_todo_list update-status <id> <status>
This command updates the status of a task with the given `id` to the specified `status`. The available status values are: done, todo, inprogress, cancel.

### Update task description
cli_todo_list update-info <id> <description>
This command updates the description of a task with the given `id` to the specified `description`.

### Update task name
cli_todo_list update-name <id> <name>
This command updates the name of a task with the given `id` to the specified `name`.

### View tasks

You can view tasks based on their status by running the following commands:

- Show all tasks: `cli_todo_list all`
- Show completed tasks: `cli_todo_list done`
- Show canceled tasks: `cli_todo_list cancel`
- Show tasks in progress: `cli_todo_list in-progress`
- Show pending tasks: `cli_todo_list todo`

### Change file name
cli_todo_list change-file-name <filename.json>
This command changes the current file name to the specified `filename.json`. This allows you to work with different todo list files.

### Other options

- `-d, --delete <ID>`: Delete a task by its ID.
- `-c, --create <new_file_name.json>`: Create a new todo list with the specified JSON file name.
- `--change-file-name <filename.json>`: Change the file name to the specified `filename.json`.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
