mod commands;
mod parsers;
mod executors;
mod repl;
mod mock;

use repl::repl;

fn main() {
    repl();
}
