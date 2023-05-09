#![allow(unused_imports)]
pub mod BlockAccess {
    use crate::define::*;

    use crate::define::*;
    use crate::BPlusTree::*;
    use crate::Buffer::Attribute;
    use crate::Buffer::BlockBuffer;
    use crate::Cache::Cache::AttrCacheTable;
    use crate::Cache::Cache::RelCacheTable;

    pub fn search(
        rel_id: i32,
        record: &Attribute,
        attr_name: &str,
        attr_val: &Attribute,
        op: i32,
    ) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn insert(rel_id: i32, record: &mut Attribute) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn rename_relation(old_name: &str, new_name: &str) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn rename_attribute(rel_name: &str, old_name: &str, new_name: &str) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn delete_relation(rel_name: &str) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn linear_search(rel_id: i32, attr_name: &str, attr_val: Attribute, op: i32) -> RecId {
        // Implementation goes here
        unimplemented!()
    }

    pub fn project(rel_id: i32, record: &Attribute) -> i32 {
        // Implementation goes here
        unimplemented!()
    }
}
use self::BlockAccess::*;
