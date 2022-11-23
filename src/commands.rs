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
    pub columns: (String,),
    pub table: String,
    pub values: (String,)
}
