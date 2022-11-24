pub enum MetaCommands {
    EXIT,
    UNKNOWN
}

pub enum QueryType {
    SELECT,
    INSERT,
    ERROR
}

// INSERT INTO TABLENAME (cols) VALUES (vals)

pub struct Query {
    pub command: QueryType,
    pub columns: Option<Vec<String>>,
    pub table: Option<String>,
    pub values: Option<Vec<String>>
}
