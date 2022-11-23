use std::io::{stdout, stdin, Write};
use super::parsers::{parse_command, parse_query};
use super::executors::{exec_cmd, exec_query};

pub fn repl() {
    let prompt: &str = "sqlite> ";
    let mut lock = stdout().lock();

    loop {
        let mut raw_cmd = String::new();
        write!(lock, "{}", prompt).unwrap();

        stdout()
            .flush()
            .expect("Flush failed");

        stdin()
            .read_line(&mut raw_cmd)
            .expect("Failed to read query");

        exec_cmd(parse_command(&raw_cmd.trim()));

        let response = exec_query(parse_query(&raw_cmd.trim()));
        write!(lock, "{}\n", response).unwrap();
    }
}
