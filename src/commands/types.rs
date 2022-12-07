use bincode;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type TableRowsCtr = u64;
pub type TableColsCtr = u8;

#[derive(Clone, Debug)]
pub enum MetaCommand {
    EXIT,
    ERROR,
    INVALIDQUERY
}

#[derive(Debug, Clone)]
pub enum DTypes {
    INT,
    VARCHAR,
    BOOL
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MockSchema {
    name: String,
    age: u8,
    is_alive: bool
}

impl MockSchema {
    pub fn new(name: String, age: u8, is_alive: bool) -> Self {
        MockSchema { name, age, is_alive }
    }
}

fn deserialize_row(page: &Vec<u8>) -> Option<MockSchema> {
    let data = bincode::deserialize(&page);
    match data {
        Ok(row) => Some(row),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    n_cols: TableColsCtr,
    n_rows: TableRowsCtr,
    cols: Vec<(String, DTypes)>,
    pub pages: Vec<Vec<u8>>
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            n_cols: 0,
            n_rows: 0,
            cols: vec![],
            pages: vec![]
        }
    }

    pub fn set_col(&mut self, col_name: &str, col_type: DTypes) -> &mut Self {
        self.cols.push((String::from(col_name), col_type));
        self.n_cols += 1;
        self
    }

    pub fn insert<T: serde::ser::Serialize>(&mut self, data: T) -> &mut Self {
        let row_op = bincode::serialize(&data);
        match row_op {
            Ok(row) => {
                self.pages.push(row);
                self.n_rows += 1;
            },
            Err(e) => { println!("{:?}", e); }
        }
        self
    }

    pub fn select(&self) -> Vec<MockSchema> {
        self.pages.iter()
            .map(deserialize_row)
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
    }
}

pub struct Db {
    pub tables: HashMap<String, Table>,
    n_tables: u64
}

impl Db {
    pub fn new() -> Self {
        let tables = HashMap::new();
        Db { tables, n_tables: 0 }
    }

    pub fn add(&mut self, table_name: &str, table: Table) -> &mut Self {
        let k = String::from(table_name);
        self.tables.insert(k, table);
        self.n_tables += 1;
        self
    }

    pub fn remove(&mut self, table_name: &str) -> &mut Self {
        self.tables.remove(table_name);
        self.n_tables -= 1;
        self
    }
}