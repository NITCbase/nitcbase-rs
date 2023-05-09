#![allow(unused_imports)]
pub mod BPlustTree {
    use crate::define::*;
    use crate::Buffer::Attribute;
    use crate::Buffer::Index;
    use crate::Buffer::InternalEntry;
    fn findLeaftoInsert(rootBlock: i32, attrVal: Attribute, attrType: i32) -> i32 {
        1
    }
    fn insertIntoLeaf(rootBlock: i32, attrName: &str, blockNum: i32, entry: Index) -> i32 {
        1
    }
    fn splitLeaf(leafBlockNum: i32, indices: &Vec<Index>) -> i32 {
        1
    }
    fn insertIntoInternal(
        relId: i32,
        attrName: &str,
        intBlockNum: i32,
        entry: InternalEntry,
    ) -> i32 {
        unimplemented!();
    }
    fn splitInternal(intBlockNum: i32, internalEntries: &Vec<InternalEntry>) -> i32 {
        unimplemented!();
    }
    fn createNewRoot(
        relId: i32,
        attrName: &str,
        attrVal: Attribute,
        lChild: i32,
        rChild: i32,
    ) -> i32 {
        1
    }
    pub fn bPlusCreate(relId: i32, attrName: &str) -> i32 {
        unimplemented!();
    }
    pub fn bPlustInsert(relId: i32, attrName: &str) -> i32 {
        unimplemented!();
    }
    pub fn bPlusSearch(relId: i32, attrName: &str, attrVal: Attribute, recordId: RecId) -> RecId {
        unimplemented!();
    }
    pub fn bPlusDestroy(relId: i32) -> i32 {
        unimplemented!();
    }
}
use self::BPlustTree::*;
