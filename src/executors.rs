use std::process;
use super::commands::{QueryType, MetaCommands, Query};
use super::mock::db;

pub fn exec_cmd(cmd: MetaCommands) -> () {
    match cmd {
        MetaCommands::EXIT => process::exit(0),
        MetaCommands::UNKNOWN => ()
    }
}

pub fn exec_query(q: Query) -> String {
    let mut db_ = db().to_vec();
    match q.command {
        QueryType::SELECT => {
            return format!("{:?}", db_);
        },
        QueryType::INSERT => {
            let last = *db_.last().unwrap();
            let last_id = last.0;
            db_.push((last_id + 1, "...", 18, 12, 50_000));
            return format!("{:?}", db_);
        },
        _ => "SQL Parsing failed.".to_string()
    }
}
