use super::commands::{MetaCommands, QueryType, Query};

pub fn parse_command(command: &str) -> MetaCommands {
    if command == ".exit" {
        return MetaCommands::EXIT
    }
    MetaCommands::UNKNOWN
}

pub fn parse_query(query: &str) -> Query {
    let q_parts: Vec<&str> = query.split_whitespace().collect();
    let err_q = Query {
        command: QueryType::ERROR,
        columns: ("".to_string(),),
        table: "".to_string(),
        values: ("".to_string(),)
    };

    if q_parts.len() == 0 {
        return err_q
    }

    let cmd = q_parts[0].to_lowercase();

    if cmd == "select" {
        return Query {
            command: QueryType::SELECT,
            columns: ("".to_string(),),
            table: "".to_string(),
            values: ("".to_string(),)
        }
    } else if cmd == "insert" {
        return Query {
            command: QueryType::INSERT,
            columns: ("".to_string(),),
            table: "".to_string(),
            values: ("".to_string(),)
        }
    }

    return err_q
}
