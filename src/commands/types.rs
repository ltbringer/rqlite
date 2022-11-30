use std::collections::HashMap;

pub type TableRowsCtr = u64;
pub type TableColsCtr = u8;

#[derive(Clone, Debug)]
pub enum MetaCommand {
    EXIT,
    ERROR,
    INVALIDQUERY
}

#[derive(Debug)]
pub enum DTypes {
    INT,
    VARCHAR
}

#[derive(Debug)]
pub enum QTypes {
    SELECT,
    INSERT
}

#[derive(Debug)]
pub struct Query {
    pub type_: QTypes,
    pub table: String,
    pub columns: Vec<String>,
    pub values: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum Command {
    M(MetaCommand),
    Q(Query)
}

#[derive(Debug)]
pub struct Table {
    n_cols: TableColsCtr,
    n_rows: TableRowsCtr,
    name: String,
    col_names: Vec<String>,
    col_types: Vec<DTypes>,
}

pub struct Db {
    tables: HashMap<String, Table>,
    n_tables: u64
}

impl Db {
    pub fn get<'a>(self, name: &str) -> Option<&'a Table> {
        self.tables.get(name)
    }

    pub fn new() -> Self {
        let tables = HashMap::new();
        Db { tables, n_tables: 0 }
    }

    pub fn add(&mut self, table_name: &str, table: Table) -> &mut Self {
        self.tables.insert(String::from(table_name), table);
        self.n_tables += 1;
        self
    }

    pub fn remove(&mut self, table_name: &str) -> &mut Self {
        self.tables.remove(table_name);
        self.n_tables -= 1;
        self
    }
}