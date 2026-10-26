# Taskit

Taskit is a command-line task manager that keeps a to-do list inside each project, rather than in
one global list somewhere else. It is still in its early stages: the repository is mostly a
skeleton and the only code so far prints a greeting, but I'm hoping to get a rudimentary CLI
working soon.

I'm using this project to learn Rust, which is why the argument parsing is written by hand instead
of using something like clap. Working out how it fits together is part of the point.

> **Note:** This is one of my first proper projects, and it's built around my own workflow, so some
> command names and TUI layouts may seem odd. If you spot a mistake, an unidiomatic bit of Rust, or
> something that could be structured better, feel free to open an issue or send a pull request.

> **Disclaimer:** I used AI (OpenCode with the Big Pickle model) to help write some of this
> documentation, mainly for structure and wording. The ideas and direction come from the mock-up
> README I wrote before asking for help, since I'm not great at documenting things.

## Planned usage

Once the commands exist, the idea is:

```shell
taskit                  # open the TUI
taskit add "Buy milk"   # add a task
taskit ls               # list all tasks
taskit done 1           # mark task 1 as done
taskit rm 1             # remove task 1
```

There's no demo yet. I'll add one when there's something worth recording.

## The task file

Tasks live in `taskit-tasks.json` in the project directory. Putting the file next to the code means
the list stays with the project. You can commit it so other people can see what's left to do, or add
it to `.gitignore` if you'd rather keep your list private. Taskit will sort the file so unfinished
tasks come first, meaning the next thing to do is near the top.

The format is still being decided. I'm set on using JSON, but the structure of the file is still
open to change.

One thing to be aware of: because the file sits in the repository, two people can edit it at the
same time and overwrite each other's changes. For now, the last write wins. If you know of a way to
let several people edit the file safely at once, I'd be happy to hear it.

## How it's put together

A few decisions worth explaining:

- No CLI framework. I want to learn how argument handling works, so I'm writing it from scratch.
  This isn't me claiming it's the better approach.
- JSON instead of a database. A task list should be small, readable, and easy to diff in Git, and a
  single JSON file does that better than an embedded database for a project this size.
- One file per project. A shared task list mixes unrelated work together, so keeping it local to
  each project makes more sense.

## What's next

1. Argument parsing.
2. Reading and writing the task file.
3. `add`, `ls`, `done` and `rm`.
4. The TUI.
5. A first release, `v0.1.0`.

## Building

You'll need a stable Rust toolchain; the project uses the 2024 edition, so Rust 1.85 or newer.
There are no other dependencies.

```shell
git clone https://github.com/Jamswert/taskit.git
cd taskit
cargo run
```

To build a release binary, use `cargo build --release`. It ends up in `target/release/taskit`.

## Contributing

Suggestions and corrections are welcome, especially about idiomatic Rust. I'm still learning, so
if something could be written more clearly or safely, I'd like to hear about it.

There isn't a strict style guide. Try to follow the style of the code around you, and be kind.
First-time contributors are welcome. By contributing, you agree that your work will be released
under the MIT Licence.

## Licence

MIT. See [LICENSE](LICENSE).
