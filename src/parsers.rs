use super::commands::{MetaCommands, QueryType};

pub fn parse_command(command: &str) -> MetaCommands {
    if command == ".exit" {
        return MetaCommands::EXIT
    }
    MetaCommands::UNKNOWN
}

pub fn parse_query(query: &str) -> QueryType {
    if query.to_lowercase() == "select" {
        return QueryType::SELECT
    } else if query.to_lowercase() == "insert" {
        return QueryType::INSERT
    }
    return QueryType::ERROR
}
