#![allow(unused_imports)]
pub mod Cache {
    use crate::define::{AttributeType, RecId};

    pub mod RelCacheTable {
        use super::CacheEntry;
        use super::RelCatEntry;
        use crate::define::*;
        use crate::Buffer::Attribute;
        use lazy_static::lazy_static;

        // Public methods
        pub fn get_rel_cat_entry(rel_id: i32, rel_cat_buf: &mut RelCatEntry) -> i32 {
            unimplemented!()
        }

        pub fn set_rel_cat_entry(rel_id: i32, rel_cat_buf: &RelCatEntry) -> i32 {
            unimplemented!()
        }

        pub fn get_search_index(rel_id: i32, search_index: &mut RecId) -> i32 {
            unimplemented!()
        }

        pub fn set_search_index(rel_id: i32, search_index: &RecId) -> i32 {
            unimplemented!()
        }

        pub fn reset_search_index(rel_id: i32) -> i32 {
            unimplemented!()
        }

        // Private field
        lazy_static! {
            static ref REL_CACHE: [CacheEntry; MAX_OPEN] = [CacheEntry::default(); MAX_OPEN];
        }
        // Private methods
        fn record_to_rel_cat_entry(
            record: &[Attribute; RELCAT_NO_ATTRS],
            rel_cat_entry: &mut RelCatEntry,
        ) {
            unimplemented!()
        }

        fn rel_cat_entry_to_record(
            rel_cat_entry: &RelCatEntry,
            record: &mut [Attribute; RELCAT_NO_ATTRS],
        ) {
            unimplemented!()
        }
    }

    pub mod AttrCacheTable {
        use super::AttrCatEntry;
        use super::CacheEntry;
        use crate::define::*;
        use crate::Buffer::Attribute;
        use lazy_static::lazy_static;

        // Public methods

        pub fn get_attr_cat_entry(
            rel_id: i32,
            attr_offset: i32,
            attr_cat_buf: &mut AttrCatEntry,
        ) -> i32 {
            unimplemented!()
        }

        pub fn set_attr_cat_entry(
            rel_id: i32,
            attr_name: &[u8; ATTR_SIZE],
            attr_cat_buf: &AttrCatEntry,
        ) -> i32 {
            unimplemented!()
        }

        pub fn get_search_index(rel_id: i32, attr_offset: i32, search_index: &mut IndexId) -> i32 {
            unimplemented!()
        }

        pub fn set_search_index(
            rel_id: i32,
            attr_name: &[u8; ATTR_SIZE],
            search_index: &IndexId,
        ) -> i32 {
            unimplemented!()
        }

        pub fn reset_search_index(rel_id: i32, attr_offset: i32) -> i32 {
            unimplemented!()
        }

        // Private field

        // Private field
        lazy_static! {
            static ref ATTR_CACHE: [CacheEntry; MAX_OPEN] = [CacheEntry::default(); MAX_OPEN];
        }

        // Private methods
        fn record_to_attr_cat_entry(
            record: &[Attribute; ATTRCAT_NO_ATTRS],
            attr_cat_entry: &mut AttrCatEntry,
        ) {
            unimplemented!()
        }

        fn attr_cat_entry_to_record(
            attr_cat_entry: &AttrCatEntry,
            record: &mut [Attribute; ATTRCAT_NO_ATTRS],
        ) {
            unimplemented!()
        }
    }
    pub mod OpenRelTable {
        use super::OpenRelTableMetaInfo;
        use crate::define::*;
        use lazy_static::lazy_static;

        // Public methods
        pub fn get_rel_id(rel_name: &[u8; ATTR_SIZE]) -> i32 {
            unimplemented!()
        }

        pub fn open_rel(rel_name: &[u8; ATTR_SIZE]) -> i32 {
            unimplemented!()
        }

        pub fn close_rel(rel_id: i32) -> i32 {
            unimplemented!()
        }

        // Private field
        lazy_static! {
            pub static ref TABLE_META_INFO: Vec<OpenRelTableMetaInfo> = Vec::default();
        }
        // Private methods
        fn get_free_open_rel_table_entry() -> i32 {
            unimplemented!()
        }
    }

    // Private types
    #[derive(Default, Clone, Copy)]
    pub struct CacheEntry {
        pub free: bool,
        pub dirty: bool,
        pub block_num: i32,
        pub time_stamp: i32,
    }

    pub struct RelCatEntry<'a> {
        pub relName: &'a str,
        pub numAttrs: i32,
        pub numRecs: i32,
        pub firstBlk: i32,
        pub lastBlk: i32,
        pub numSlotsPerBlk: i32,
    }

    pub struct RelCacheEntry<'a> {
        relCatEntry: RelCatEntry<'a>,
        dirty: bool,
        recId: RecId,
        searchIndex: RecId,
    }

    pub struct AttrCatEntry<'a> {
        relName: &'a str,
        attrName: &'a str,
        attrType: AttributeType,
        primaryFlag: bool,
        rootBlock: i32,
        offset: usize,
    }

    #[derive(Clone)]
    pub struct OpenRelTableMetaInfo {
        pub free: bool,
        pub relName: String,
    }
    impl OpenRelTableMetaInfo {
        pub fn default() -> OpenRelTableMetaInfo {
            OpenRelTableMetaInfo {
                free: (false),
                relName: ("".to_string()),
            }
        }
    }
}
use self::Cache::*;
