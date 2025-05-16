use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Integer(i32),
    Text(String),
    Bool(bool),
    Tag(String),
}

pub struct Document {
    pub id: String,
    pub attributes: HashMap<String, Vec<Value>>,
}
impl Document {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            attributes: HashMap::new(),
        }
    }
    pub fn attribute(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.attributes
            .entry(key.into())
            .or_default()
            .push(value.into());
        self
    }
}
