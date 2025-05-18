use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

pub type EntryIndex = u32;
pub type ShardingValue = u64;
pub type DocumentIdentifier = Arc<str>;

pub struct Entry {
    pub index: EntryIndex,
    pub name: DocumentIdentifier,
    pub sharding: ShardingValue,
}
pub struct PersistedCollection {
    pub entries: Vec<Entry>,
}
pub struct Collection {
    entries_by_index: HashMap<EntryIndex, DocumentIdentifier>,
    entries_by_name: HashMap<DocumentIdentifier, EntryIndex>,
    sharding: BTreeMap<ShardingValue, HashSet<EntryIndex>>,
}
