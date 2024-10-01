# TodoList-rs 📑

Simple TODO List in Rust

# Usage 📎:

1. Clone the repo:
   `https://github.com/Hkmori15/TodoList-rs.git`
2. Build a binary: `cargo build --release`
3. For add task to todos list use command: `cargo run -- add "Something important"`
4. For deleting task from todos list use command: `cargo run -- delete 1`. Number 1 is id task you can see id task in file `todos.json`
5. For mark task as done use command: `cargo run -- done 1`
6. For load exists task use command: `cargo run -- load todos.json`
7. For view a list task use command: `cargo run -- list`
8. For search task use a keyword: `cargo run -- search "important"`
9. For view done and not done tasks use command: `cargo run -- filter --done` and `cargo run -- filter --not-done`
