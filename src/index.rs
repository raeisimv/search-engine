use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

pub type EntryIndex = u32;
pub type AttributeIndex = u8;
pub type ShardingValue = u64;
pub type DocumentIdentifier = Arc<str>;

pub struct Entry {
    pub index: EntryIndex,
    pub name: DocumentIdentifier,
    pub shard: ShardingValue,
}
pub struct Attribute {
    pub index: AttributeIndex,
    pub name: Arc<str>,
}
pub struct PersistedCollection {
    pub entries: Vec<Entry>,
    pub attributes: Vec<Attribute>,
}
pub struct Collection {
    entries_by_index: HashMap<EntryIndex, DocumentIdentifier>,
    entries_by_name: HashMap<DocumentIdentifier, EntryIndex>,
    attributes_by_index: HashMap<AttributeIndex, Arc<str>>,
    attributes_by_name: HashMap<Arc<str>, AttributeIndex>,
    sharding: BTreeMap<ShardingValue, HashSet<EntryIndex>>,
}
impl From<PersistedCollection> for Collection {
    fn from(value: PersistedCollection) -> Self {
        let mut entries_by_name = HashMap::with_capacity(value.entries.len());
        let mut entries_by_index = HashMap::with_capacity(value.entries.len());
        let mut sharding: BTreeMap<ShardingValue, HashSet<EntryIndex>> = BTreeMap::new();
        for x in value.entries {
            entries_by_index.insert(x.index, x.name.clone());
            entries_by_name.insert(x.name.clone(), x.index);
            sharding.entry(x.shard).or_default().insert(x.index);
        }
        Collection {
            entries_by_index,
            entries_by_name,
            attributes_by_index: Default::default(),
            attributes_by_name: Default::default(),
            sharding,
        }
    }
}

type ValueIndex = u8;
pub struct BooleanIndex {
    pub content: HashMap<bool, HashMap<AttributeIndex, HashMap<EntryIndex, HashSet<ValueIndex>>>>,
}
impl BooleanIndex {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        entry_index: EntryIndex,
        attribute_index: AttributeIndex,
        value_index: ValueIndex,
        term: bool,
    ) {
        let term_postings = self.content.entry(term).or_default();
        let attribute_postings = term_postings.entry(attribute_index).or_default();
        let entry_postings = attribute_postings.entry(entry_index).or_default();
        entry_postings.insert(value_index);
    }
    pub fn delete(&mut self, entry_index: EntryIndex) -> bool {
        let mut changed = false;
        for (_, postings) in self.content.iter_mut() {
            for (_, postings) in postings.iter_mut() {
                changed |= postings.remove(&entry_index).is_some();
            }
        }

        changed
    }
}
