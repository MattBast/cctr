# cctr

cctr is a copy of the unix command line tool `tr`. It was inspired by this [Coding Challenge](https://codingchallenges.fyi/challenges/challenge-tr). 

> [!WARNING]
> This project does not implement every feature that `tr` includes. The missing functionality is the `-C`, `-c` and `-u` flags and the "graph", "ideogram", "phonogram", "print", "punct", "rune", "special" and "xdigit" classes.

> [!NOTE]
> You will need to have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html) on your machine to run the below snippets.

To run the tool use the command `cargo run -q --` and then the two strings you'd like to translate. You'll be prompted for a word to translate using the strings you provided. For example if you run:
```bash
cargo run -q -- c C
```
and type `coding challenge` when prompted, you'' receive an output of `Coding Challenge`.

You can also use stdin to pipe text into the tool:
```bash
echo "coding challenge" | cargo run -q -- c C
```

And finally to get help on all the options available, try this command:
```bash
cargo run -- --help
```