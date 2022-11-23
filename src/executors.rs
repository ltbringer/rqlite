use std::process;
use super::commands::{QueryType, MetaCommands};

pub fn exec_query(query: QueryType) -> String {
    match query {
        QueryType::SELECT => "Select columns from table.".to_string(),
        QueryType::INSERT => "Insert into table.".to_string(),
        _ => "SQL Parsing failed.".to_string()
    }
}

pub fn exec_cmd(cmd: MetaCommands) -> () {
    match cmd {
        MetaCommands::EXIT => process::exit(0),
        MetaCommands::UNKNOWN => ()
    }
}
