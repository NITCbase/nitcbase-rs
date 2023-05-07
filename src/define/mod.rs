#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt;

pub static DISK_PATH: &str = "../Disk/disk"; // Path to disk
pub static DISK_RUN_COPY_PATH: &str = "../Disk/disk_run_copy"; // Path to run copy of the disk
pub static FILES_PATH: &str = "../Files/"; // Path to Files directory;;
pub static INPUT_FILES_PATH: &str = "../Files/Input_Files/"; // Path to Input_Files directory inside the Files directory
pub static OUTPUT_FILES_PATH: &str = "../Files/Output_Files/"; // Path to Output_Files directory inside the Files directory
pub static BATCH_FILES_PATH: &str = "../Files/Batch_Execution_Files/"; // Path to Batch_Execution_Files directory inside the Files directory

pub const BLOCK_SIZE: usize = 2048; // Size of Block in bytes
pub const ATTR_SIZE: usize = 16; // Size of an attribute in bytes
pub const DISK_SIZE: usize = 16 * 1024 * 1024; // Size of Disk in bytes
pub const HEADER_SIZE: usize = 32; // Size of Header of a block in bytes (not including slotmap)
pub const LCHILD_SIZE: usize = 4; // Size of field Lchild in bytes
pub const RCHILD_SIZE: usize = 4; // Size of field Rchild in bytes
pub const PBLOCK_SIZE: usize = 4; // Size of field Pblock in bytes
pub const BLOCKNUM_SIZE: usize = 4; // Size of field BlockNum in bytes
pub const SLOTNUM_SIZE: usize = 4; // Size of field SlotNum in bytes
pub const INDEX_BLOCK_UNUSED_BYTES: usize = 8; // Size of unused field in index block (in bytes)
pub const INTERNAL_ENTRY_SIZE: usize = 24; // Size of an Internal Index Entry in the Internal Index Block (in bytes)
pub const LEAF_ENTRY_SIZE: usize = 32; // Size of an Leaf Index Entry in the Leaf Index Block (in bytes)

pub const DISK_BLOCKS: usize = 8192; // Number of block in disk
pub const BUFFER_CAPACITY: usize = 32; // Total number of blocks available in the Buffer (Capacity of the Buffer in blocks)
pub const MAX_OPEN: usize = 12; // Maximum number of relations allowed to be open and cached in Cache Layer.
pub const BLOCK_ALLOCATION_MAP_SIZE: usize = 4; // Number of blocks given for Block Allocation Map in the disk

pub const RELCAT_NO_ATTRS: usize = 6; // Number of attributes present in one entry / record of the Relation Catalog
pub const ATTRCAT_NO_ATTRS: usize = 6; // Number of attributes present in one entry / record of the Attribute Catalog

pub const RELCAT_BLOCK: usize = 4; // Disk block number for the block of Relation Catalog
pub const ATTRCAT_BLOCK: usize = 5; // Disk block number for the first block of Attribute Catalog

pub const NO_OF_ATTRS_RELCAT_ATTRCAT: usize = 6; // Common variable to indicate the number of attributes present in one entry of Relation Catalog / Attribute Catalog
pub const SLOTMAP_SIZE_RELCAT_ATTRCAT: usize = 20; // Size of slotmap in both Relation Catalog and Attribute Catalog

pub const SLOT_OCCUPIED: char = '1'; // Value to mark a slot in Slotmap as Occupied
pub const SLOT_UNOCCUPIED: char = '0'; // Value to mark a slot in Slotmap as Unoccupied

pub const RELCAT_RELID: usize = 0; // Relid for Relation catalog
pub const ATTRCAT_RELID: usize = 1; // Relid for Attribute catalog

pub const RELCAT_SLOTNUM_FOR_RELCAT: usize = 0; // Slot number for relation catalog in relation catalog
pub const RELCAT_SLOTNUM_FOR_ATTRCAT: usize = 1; // Slot number for attribute catalog in relation catalog

pub const INVALID_BLOCKNUM: i32 = -1; // Indicates the Block number as Invalid.

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum AttributeType {
    NUMBER, // for an integer or a floating point number 0
    STRING, // 1
}

#[allow(non_camel_case_types)]
pub enum ConditionalOperators {
    EQ, // =
    LE, // <=
    LT, // <
    GE, // >=
    GT, // >
    NE, // !=
}

#[allow(non_camel_case_types)]
pub enum BlockType {
    REC,          // record block
    IND_INTERNAL, // internal index block
    IND_LEAF,     // leaf index block
    UNUSED_BLK,   // unused block
    BMAP,         // block allocation map
}

#[allow(non_camel_case_types)]
pub enum OpenRelationEntryStatus {
    OCCUPIED, //1
    FREE,     //0
}

// Indexes for Relation Catalog Attributes
#[allow(non_camel_case_types)]
pub enum RelCatFieldIndex {
    RELCAT_REL_NAME_INDEX,           // Relation Name
    RELCAT_NO_ATTRIBUTES_INDEX,      // #Attributes
    RELCAT_NO_RECORDS_INDEX,         // #Records
    RELCAT_FIRST_BLOCK_INDEX,        // First Block
    RELCAT_LAST_BLOCK_INDEX,         // Last Block
    RELCAT_NO_SLOTS_PER_BLOCK_INDEX, // #Slots
}

impl RelCatFieldIndex {
    pub fn id(&self) -> usize {
        match self {
            RelCatFieldIndex::RELCAT_REL_NAME_INDEX => 0,
            RelCatFieldIndex::RELCAT_NO_ATTRIBUTES_INDEX => 1,
            RelCatFieldIndex::RELCAT_NO_RECORDS_INDEX => 2,
            RelCatFieldIndex::RELCAT_FIRST_BLOCK_INDEX => 3,
            RelCatFieldIndex::RELCAT_LAST_BLOCK_INDEX => 4,
            RelCatFieldIndex::RELCAT_NO_SLOTS_PER_BLOCK_INDEX => 5,
        }
    }
}

// Indexes for Attribute Catalog Attributes
#[allow(non_camel_case_types)]
pub enum AttrCatFieldIndex {
    ATTRCAT_REL_NAME_INDEX,     // Relation Name
    ATTRCAT_ATTR_NAME_INDEX,    // Attribute Name
    ATTRCAT_ATTR_TYPE_INDEX,    // Attribute Type
    ATTRCAT_PRIMARY_FLAG_INDEX, // Primary Flag
    ATTRCAT_ROOT_BLOCK_INDEX,   // Root Block
    ATTRCAT_OFFSET_INDEX,       // Offset
}

impl AttrCatFieldIndex {
    pub fn id(&self) -> usize {
        match self {
            AttrCatFieldIndex::ATTRCAT_REL_NAME_INDEX => 0,
            AttrCatFieldIndex::ATTRCAT_ATTR_TYPE_INDEX => 1,
            AttrCatFieldIndex::ATTRCAT_ATTR_NAME_INDEX => 2,
            AttrCatFieldIndex::ATTRCAT_PRIMARY_FLAG_INDEX => 3,
            AttrCatFieldIndex::ATTRCAT_ROOT_BLOCK_INDEX => 4,
            AttrCatFieldIndex::ATTRCAT_OFFSET_INDEX => 5,
        }
    }
}
#[allow(non_camel_case_types)]
pub enum ErrorType {
    FAILURE,               // -1
    EXIT,                  //-100
    OUTOFBOUND,            // Out of bound
    FREESLOT,              // Free slot
    NOINDEX,               // No index
    DISKFULL,              // Insufficient space in Disk
    INVALIDBLOCK,          // Invalid block
    RELNOTEXIST,           // Relation does not exist
    RELEXIST,              // Relation already exists
    ATTRNOTEXIST,          // Attribute does not exist
    ATTREXIST,             // Attribute already exists
    CACHEFULL,             // Cache is full
    RELNOTOPEN,            // Relation is not open
    NATTRMISMATCH,         // Mismatch in number of attributes
    DUPLICATEATTR,         // Duplicate attributes found
    RELOPEN,               // Relation is open
    ATTRTYPEMISMATCH,      // Mismatch in attribute type
    INVALID,               // Invalid index or argument
    MAXRELATIONS,          // Maximum number of relations already present
    MAXATTRS,              // Maximum number of attributes allowed for a relation is 125
    NOTPERMITTED,          // Operation not permitted
    NOTFOUND,              // Search for requested record unsuccessful
    BLOCKNOTINBUFFER,      // Block not found in buffer
    INDEX_BLOCKS_RELEASED, // Due to insufficient disk space, index blocks have been released from the disk
    CAPTURE_FAILURE,
    IO_ERROR(std::io::Error), //
}
impl ErrorType {
    pub fn print_error(&self) {
        match &self {
            ErrorType::CAPTURE_FAILURE => println!("Error: Regex Capture error"),
            ErrorType::FAILURE => println!("Error: Command Failed"),
            ErrorType::EXIT => println!("Error: Exit"),
            ErrorType::OUTOFBOUND => println!("Error: Out of bound"),
            ErrorType::FREESLOT => println!("Error: Free slot"),
            ErrorType::NOINDEX => println!("Error: No index"),
            ErrorType::DISKFULL => println!("Error: Insufficient space in disk"),
            ErrorType::INVALIDBLOCK => println!("Error: Invalid block"),
            ErrorType::RELNOTEXIST => println!("Error: Relation does not exist"),
            ErrorType::RELEXIST => println!("Error: Relation already exists"),
            ErrorType::ATTRNOTEXIST => println!("Error: Attribute does not exist"),
            ErrorType::ATTREXIST => println!("Error: Attribute already exists"),
            ErrorType::CACHEFULL => println!("Error: Cache is full"),
            ErrorType::RELNOTOPEN => println!("Error: Relation is not open"),
            ErrorType::NATTRMISMATCH => println!("Error: Mismatch in number of attributes"),
            ErrorType::DUPLICATEATTR => println!("Error: Duplicate attributes found"),
            ErrorType::RELOPEN => println!("Error: Relation is open"),
            ErrorType::ATTRTYPEMISMATCH => println!("Error: Mismatch in attribute type"),
            ErrorType::INVALID => println!("Error: Invalid index or argument"),
            ErrorType::MAXRELATIONS => {
                println!("Error: Maximum number of relations already present")
            }
            ErrorType::MAXATTRS => {
                println!("Error: Maximum number of attributes allowed for a relation is 125")
            }
            ErrorType::NOTPERMITTED => println!("Error: This operation is not permitted"),
            ErrorType::NOTFOUND => println!("Error: Search for requested record unsuccessful"),
            ErrorType::BLOCKNOTINBUFFER => println!("Error: Block not found in buffer"),
            ErrorType::INDEX_BLOCKS_RELEASED => {
                println!("Warning: Operation succeeded, but some indexes had to be dropped")
            }
            ErrorType::IO_ERROR(io_error) => println!("I/O Error: {}", io_error),
        }
    }
}

pub static TEMP: &str = ".temp"; // Used; for internal purposes;

// Global variables for B+ Tree Layer
pub const MAX_KEYS_INTERNAL: usize = 100; // Maximum number of keys allowed in an Internal Node of a B+ tree
pub const MIDDLE_INDEX_INTERNAL: usize = 50; // Index of the middle element in an Internal Node of a B+ tree
pub const MAX_KEYS_LEAF: usize = 63; // Maximum number of keys allowed in a Leaf Node of a B+ tree
pub const MIDDLE_INDEX_LEAF: usize = 31; // Index of the middle element in a Leaf Node of a B+ tree

// Name strings for Relation Catalog and Attribute Catalog (as it is stored in the Relation catalog)
pub static RELCAT_RELNAME: &str = "RELATIONCAT";
pub static ATTRCAT_RELNAME: &str = "ATTRIBUTECAT";

// Relation; Catalog attribute name strings
pub static RELCAT_ATTR_RELNAME: &str = "RelName";
pub static RELCAT_ATTR_NO_ATTRIBUTES: &str = "#Attributes";
pub static RELCAT_ATTR_NO_RECORDS: &str = "#Records";
pub static RELCAT_ATTR_FIRST_BLOCK: &str = "FirstBlock";
pub static RELCAT_ATTR_LAST_BLOCK: &str = "LastBlock";
pub static RELCAT_ATTR_NO_SLOTS: &str = "#Slots";

// Attribte; Catalog attribute name strings
pub static ATTRCAT_ATTR_RELNAME: &str = "RelName";
pub static ATTRCAT_ATTR_ATTRIBUTE_NAME: &str = "AttributeName";
pub static ATTRCAT_ATTR_ATTRIBUTE_TYPE: &str = "AttributeType";
pub static ATTRCAT_ATTR_PRIMARY_FLAG: &str = "PrimaryFlag";
pub static ATTRCAT_ATTR_ROOT_BLOCK: &str = "RootBlock";
pub static ATTRCAT_ATTR_OFFSET: &str = "Offset";

pub struct CharArray16([char; 16]);

impl fmt::Display for CharArray16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &ch in self.0.iter() {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}
