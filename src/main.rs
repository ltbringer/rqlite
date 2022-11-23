use std::io::{stdout, stdin, Write};

fn neat_query(strings: Vec<String>) -> String {
    let clean_strings = strings.iter()
        .map(|s| adjust_whitespace(s))
        .collect::<Vec<_>>();
    clean_strings.join(" ")
}

fn adjust_whitespace(text: &str) -> String {
    let texts: Vec<_> = text.split_whitespace().collect();
    texts.join(" ")
}

fn main() {
    let prompt: &str = "sqlite> ";
    let mut lock = stdout().lock();
    loop {
        write!(lock, "{}", prompt).unwrap();
        stdout().flush().expect("Flush failed");
        let lines = stdin().lines();
        let raw_query = lines
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        let query = neat_query(raw_query);
        write!(lock, "{}\n", query).unwrap();
    }
}
