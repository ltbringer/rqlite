mod commands;
mod parsers;
mod executors;
mod repl;

use repl::repl;

fn main() {
    repl();
}
