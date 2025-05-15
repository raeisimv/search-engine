use crate::errors::MyResult;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum SchemaError {
    ShardingAttributeNotSet,
    UnknownShardingAttribute,
}
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    BOOLEAN,
    INTEGER,
    TAG,
    TEXT,
}

pub struct Schema {
    pub attributes: HashMap<String, Kind>,
    pub shard_by: String,
}
impl Schema {
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::default()
    }
}

#[derive(Default)]
pub struct SchemaBuilder {
    attributes: HashMap<String, Kind>,
    shard_by: Option<String>,
}
impl SchemaBuilder {
    pub fn shard_by<S: Into<String>>(mut self, shard_by: S) -> Self {
        self.shard_by = Some(shard_by.into());
        self
    }
    pub fn attribute<S: Into<String>>(&mut self, name: S, kind: Kind) -> &mut Self {
        self.attributes.insert(name.into(), kind);
        self
    }
    pub fn build(mut self) -> MyResult<Schema, SchemaError> {
        let shard_by = self
            .shard_by
            .take()
            .ok_or(SchemaError::ShardingAttributeNotSet)?;
        if !self.attributes.contains_key(&shard_by) {
            return Err(SchemaError::UnknownShardingAttribute);
        }
        Ok(Schema {
            attributes: self.attributes,
            shard_by,
        })
    }
}
