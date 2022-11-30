use regex::{Regex, Captures};
use std::collections::HashMap;
use super::types::{Command, MetaCommand, Query, QTypes};

pub struct Parser {
    commands: HashMap<String, MetaCommand>,
}

pub static ALL_COLUMNS: &str = "*";
static SELECT_COMMAND_STR: &str = "select";
static INSERT_COMMAND_STR: &str = "insert";
static SELECT_PATTERN: &str = r"select\s+(.+)\s+from\s+([a-zA-Z_-]+)";
static INSERT_PATTERN: &str = r"insert into ([a-zA-Z_-]+) \((.+)\) values \((.+)\)";

impl Parser {
    fn new() -> Self {
        let commands: HashMap<String, MetaCommand> = HashMap::new();
        Parser { commands }
    }

    pub fn register(&mut self, text: &str, command: MetaCommand) -> &mut Self {
        self.commands.insert(String::from(text), command);
        self
    }

    fn parse_meta_command(&self, text: String) -> Command {
        let cmd = self.commands.get(&text);
        if let Some(c) = cmd {
            return Command::M(c.clone())
        }
        return Command::M(MetaCommand::ERROR);
    }

    pub fn parse(&self, text: String) -> Command {
        let t = text.to_lowercase();
        if t.starts_with(".") {
            self.parse_meta_command(t)
        } else {
            parse_query(t)
        }
    }

    pub fn build() -> Self {
        let mut parser = Parser::new();
        parser.register(".exit", MetaCommand::EXIT);
        parser
    }
}

fn vectorize(text: &str) -> Vec<String> {
    if text == ALL_COLUMNS {
        return vec![String::from(ALL_COLUMNS)];
    }
    text.split_whitespace()
        .into_iter()
        .map(String::from)
        .collect()
}

fn build_select_query(captures: Captures) -> Command {
    if let (Some(table), Some(columns)) = (captures.get(2), captures.get(1)) {
        let query = Query {
            type_: QTypes::SELECT,
            table: String::from(table.as_str()),
            columns: vectorize(columns.as_str()),
            values: None
        };
        return Command::Q(query);
    }
    return Command::M(MetaCommand::INVALIDQUERY)
}

/**
 * test: insert into table (name age married) values (jack, 21, T)
 */
fn build_insert_query(captures: Captures) -> Command {
    if let (Some(table), Some(columns), Some(values)) = (captures.get(1), captures.get(2), captures.get(3)) {
        let query = Query {
            type_: QTypes::INSERT,
            table: String::from(table.as_str()),
            columns: vectorize(columns.as_str()),
            values: Some(vectorize(values.as_str()))
        };
        return Command::Q(query);
    }
    return Command::M(MetaCommand::INVALIDQUERY)
}

fn build_query(text: String, pattern: &'static str, qbuilder: fn(Captures) -> Command) -> Command {
    println!("{} {}", text, pattern);
    let re = Regex::new(pattern).unwrap();
    let captures = re.captures(&text);
    match captures {
        Some(c) => qbuilder(c),
        _ => Command::M(MetaCommand::INVALIDQUERY)
    }
}

fn parse_query(text: String) -> Command {
    if text.starts_with(SELECT_COMMAND_STR) {
        build_query(text, SELECT_PATTERN, build_select_query)
    } else if text.starts_with(INSERT_COMMAND_STR) {
        build_query(text, INSERT_PATTERN, build_insert_query)
    } else {
        Command::M(MetaCommand::INVALIDQUERY)
    }
}