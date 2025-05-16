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
    pub fn attribute<S: Into<String>>(mut self, name: S, kind: Kind) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_schema() {
        let x = Schema::builder()
            .attribute("name", Kind::TEXT)
            .attribute("age", Kind::INTEGER)
            .attribute("phones", Kind::TEXT)
            .attribute("type", Kind::TAG)
            .shard_by("name")
            .build();

        assert!(x.is_ok());
    }
    #[test]
    fn should_fail_when_shard_is_not_set() {
        let x = Schema::builder()
            .attribute("name", Kind::TEXT)
            .attribute("age", Kind::INTEGER)
            .build();
        assert!(x.is_err());
    }
    #[test]
    fn should_fail_with_a_wrong_sharding_attribute() {
        let x = Schema::builder()
            .attribute("name", Kind::TEXT)
            .attribute("age", Kind::INTEGER)
            .attribute("phones", Kind::TEXT)
            .shard_by("email") // does not exit in attributes
            .build();
        assert!(x.is_err());
    }
}
