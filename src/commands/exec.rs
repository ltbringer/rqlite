use std::process::exit;
use super::types::{Command, MetaCommand, Query, QTypes, Table, Db, MockSchema};

pub fn run_command(cmd: Command, db: &mut Db) {
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

fn print_table(table: Option<Vec<MockSchema>> ) {
    match table {
        Some(t) => { println!("{:#?}", t); },
        _ => { println!("No data found!"); }
    }
}

fn run_query(q: Query, db: &mut Db) -> Option<Vec<MockSchema>>  {
    match q {
        Query {
            type_: QTypes::SELECT, 
            table,
            columns,
            ..
        } => select(&db, table, columns),
        Query {
            type_: QTypes::INSERT,
            table,
            columns,
            values
        } => insert(db, table, columns, values)
    }
}

fn insert(db: &mut Db, table: String, _: Vec<String>, values: Option<Vec<String>>) -> Option<Vec<MockSchema>> {
    if let (Some(t), Some(vals)) = (db.tables.get_mut(&table), values) {
        let r_name = String::from(&vals[0]);
        let r_age = vals[1].parse::<u8>().unwrap();
        let r_is_alive = vals[2].parse::<bool>().unwrap();
        let row = MockSchema::new(r_name, r_age, r_is_alive);
        t.insert(row);
        Some(t.select())
    } else {
        None
    }
}

fn select<'a>(db: &Db, table: String, _: Vec<String>) -> Option<Vec<MockSchema>> {
    if let Some(t) = db.tables.get(&table) {
        if t.pages.len() == 0 {
            return None
        }
        Some(t.select())
    } else {
        None
    }
}
