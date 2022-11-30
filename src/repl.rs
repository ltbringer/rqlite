use std::io::{stdout, stdin, Write};
use super::commands::parser::Parser;
use super::commands::exec::run_command;

pub fn repl() {
    let prompt: &str = "sqlite> ";
    let mut lock = stdout().lock();
    let parser = Parser::build();

    loop {
        let mut raw_cmd = String::new();
        write!(lock, "{}", prompt).unwrap();

        stdout()
            .flush()
            .expect("Flush failed");

        stdin()
            .read_line(&mut raw_cmd)
            .expect("Failed to read query");

        raw_cmd = String::from(raw_cmd.trim());
        let cmd = parser.parse(raw_cmd);
        run_command(cmd);
    }
}
