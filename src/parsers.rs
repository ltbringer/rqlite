use super::commands::{MetaCommands, QueryType, Query};

pub fn parse_command(command: &str) -> MetaCommands {
    if command == ".exit" {
        return MetaCommands::EXIT
    }
    MetaCommands::UNKNOWN
}

pub fn own_str_vec(parts: &Vec<&str>, start: usize, end: Option<usize>) -> Vec<String> {
    let _end = match end {
        Some(e) => e,
        None => parts.len()
    };
    String::from(&parts[start.._end].join(" "))
            .split_whitespace()
            .map(|word| word.to_owned())
            .collect::<Vec<String>>()
}

pub fn parse_query(query: &str) -> Query {
    let q_parts: Vec<&str> = query.split_whitespace().collect();
    let err_q = Query {
        command: QueryType::ERROR,
        columns: None,
        table: None,
        values: None
    };

    if q_parts.len() == 0 {
        return err_q
    }

    let cmd = q_parts[0].to_lowercase();

    if cmd == "select" {
        // select * from table
        let sep = q_parts.iter().position(|&word| word == "from").unwrap();
        let table = q_parts[sep + 1].to_owned();
        let columns = own_str_vec(&q_parts, 1, Some(sep));
        return Query {
            command: QueryType::SELECT,
            columns: Some(columns),
            table: Some(table),
            values: None
        }
    } else if cmd == "insert" {
        // insert into table name salary values bob 36000
        let table = q_parts[2].to_owned();
        let sep = q_parts.iter().position(|&word| word == "values").unwrap() + 1;
        let columns = own_str_vec(&q_parts, 3, Some(sep - 1));
        let values = own_str_vec(&q_parts, sep, None);

        return Query {
            command: QueryType::INSERT,
            columns: Some(columns),
            table: Some(table),
            values: Some(values)
        }
    }

    return err_q
}
