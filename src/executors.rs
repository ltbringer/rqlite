use std::process;
use super::commands::{QueryType, MetaCommands, Query};
use super::mock::{db, RecordFactory, TDTypes};

pub fn exec_cmd(cmd: MetaCommands) -> () {
    match cmd {
        MetaCommands::EXIT => process::exit(0),
        MetaCommands::UNKNOWN => ()
    }
}

fn select<'a>(cols: Vec<String>, db: &'a mut Vec<RecordFactory>) -> Vec<RecordFactory<'a>> {
    db.iter().map(|r| {
        let mut record = RecordFactory::new();
        for column in cols.iter() {
            match r.get(&column).unwrap() {
                TDTypes::Int(v) => record.set(&column, TDTypes::Int(*v)),
                TDTypes::Str(v) => record.set(&column, TDTypes::Str(*v))
            };
        }
        record
    }).collect::<Vec<RecordFactory>>()
}

pub fn exec_query(q: Query) -> String {
    let mut db_ = db();
    match q.command {
        QueryType::SELECT => {
            if let Some(c) = q.columns {
                if (c.len() == 1 && c[0] == "*") || (c.len() == 3) {
                    return format!("{:#?}", db_);
                } else {
                    return format!("{:#?}", select(c, &mut db_))
                }
            }
            return format!("{:#?}", db_);
        },
        QueryType::INSERT => {
            let last = db_.last().unwrap();
            let last_id = match last.get("id").unwrap() {
                TDTypes::Int(v) => *v,
                _ => 0
            };
            if let Some(v) = q.values {
                let id = TDTypes::Int(last_id + 1);
                let name = TDTypes::Str(&v[0]);
                let salary = TDTypes::Int(v[1].parse::<u32>().unwrap());
                let record = RecordFactory::new()
                    .set("id", id)
                    .set("name", name)
                    .set("salary", salary)
                    .clone();

                db_.push(record);
                return format!("{:#?}", db_);
            }
            return format!("{:#?}", db_);
        },
        _ => "SQL Parsing failed.".to_string()
    }
}
