use std::process::exit;
use super::types::{Command, MetaCommand, Query, QTypes, Table, Db};

pub fn run_command(cmd: Command, db: Db) {
    match cmd {
        Command::M(m) => run_meta_command(m),
        Command::Q(q) => print_table(run_query(q, db))
    }
}

fn run_meta_command(m: MetaCommand) {
    match m {
        MetaCommand::ERROR => println!("Invalid command!"),
        MetaCommand::EXIT => exit(0),
        MetaCommand::INVALIDQUERY => println!("Failed to parse sql!")
    }
}

fn print_table(t: &Table) {
    println!("{:#?}", t);
}

fn run_query<'a>(q: Query, db: Db) -> &'a Table {
    match q {
        Query {
            type_: QTypes::SELECT, 
            table,
            columns,
            ..
        } => select(db, table, columns),
        Query {
            type_: QTypes::INSERT,
            table,
            columns,
            values
        } => insert(db, table, columns, values)
    }
}

fn insert<'a>(db: Db, table: String, columns: Vec<String>, values: Option<Vec<String>>) -> &'a Table {

}

fn select<'a>(db: Db, table: String, columns: Vec<String>) -> &'a Table {

}