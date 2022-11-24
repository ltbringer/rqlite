use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TDTypes <'a> {
    Str(&'a str),
    Int(u32)
}

#[derive(Debug, Clone)]
pub struct RecordFactory <'a> {
    pub m: HashMap<String, TDTypes<'a>>
}

impl<'a> RecordFactory <'a> {
    pub fn new() -> Self {
        RecordFactory { m: HashMap::new() }
    }

    pub fn set(&mut self, k: &str, v: TDTypes<'a>) -> &mut Self {
        self.m.insert(String::from(k), v);
        return self;
    }

    pub fn get(&self, k: &str) -> Option<&TDTypes> {
        self.m.get(k)
    }
}

pub fn db<'a> () -> Vec<RecordFactory<'a>> {
    let r1 = RecordFactory::new()
        .set("id", TDTypes::Int(1))
        .set("name", TDTypes::Str("Jack"))
        .set("salary", TDTypes::Int(35000))
        .clone();
    let r2 = RecordFactory::new()
        .set("id", TDTypes::Int(2))
        .set("name", TDTypes::Str("Molly"))
        .set("salary", TDTypes::Int(45000))
        .clone();
    vec![r1, r2]
}
