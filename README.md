## details

the `task manager CLI` is a command-line tool designed to efficiently manage tasks using the terminal.

it allows users to add, list, retrieve and delete tasks. tasks are then stored in a JSON file, ensuring persistence across sessions.

## installation

1. **clone the repository:**

```sh
 git clone https://github.com/g4titanx/task-manager-cli.git
 cd task-manager-cli
```

2. **build the application:**

```sh
cargo build --release
```

## usage

to run the application:

```sh
cargo run -- [COMMAND] [OPTIONS]
```

## commands

1. use the `add` command to add a new task

```sh
cargo run -- add "TaskName" "Objective 1" "Objective 2"
```

2. use the `list` command to list all tasks

```sh
cargo run -- list
```

3. use the `get` command to get a specific task

```sh
cargo run -- get "TaskName"
```

4. use the `update` command to update a specific task objective

```sh
cargo run -- update "TaskName" 0
```

4. use the `delete` command to delete a specific task

```sh
cargo run -- delete "TaskName"
```

## file structure

the tasks are stored in a JSON file (tasks.json) in the same directory where the CLI is executed. ensure this file is present and readable/writable by the application.

## code overview

the core functionality is defined in the library. it includes methods to add, list, get, update and delete tasks, as well as to load and save tasks to a JSON file.
the CLI is implemented using the clap crate, which provides a simple interface to define commands and arguments. the main application uses the task manager library to perform the desired operations based on user input.

## contribution

feel free to fork this repository, make your changes, and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.
