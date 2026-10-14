# bubblewrap-helper

A small Rust utility (work in progress) to grab program file interactions and piped them into a text file for comprehensive bubblewrap scripting.

Status
------
This project is a work in progress. The current repository contains the initial Rust source code, but several planned features remain to be implemented.

Planned work / roadmap
----------------------
- Implement a Ratatui-based TUI for interactive use.
- Add search filters for the PID/process list (name, regex(?), pid, user) so filtering and selection is easier and faster.
- Reduce or remove on using the Linux shell.
- Improve error handling, logging, and testing.

Build / compile instructions
----------------------------
Prerequisites

1) Clone the repository:
   git clone https://github.com/Ea-Al-Az/bubblewrap-helper.git
   cd bubblewrap-helper

2) Build in debug mode:
   cargo build

3) Build an release binary:
   cargo build --release
   The release binary will be at:
   target/release/bubblewrap-helper

4) Run with Cargo:
   cargo run -- [args]

Testing and development
-----------------------
- Use `cargo run` while iterating on the code.
- Add tests and run them with:
  cargo test

Contributing
------------
Contributions and suggestions are very much welcome. If you want to help implement the TUI, search filters, make the script overall less dependent on other libraries, open an issue or a PR.
