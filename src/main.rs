mod repl;
mod help;
mod commands;

use repl::repl;
use help::help_message;

fn main() {
    help_message();
    repl();
}
