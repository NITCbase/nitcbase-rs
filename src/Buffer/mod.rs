#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use lazy_static::lazy_static;
use std::mem;

use crate::define::*;
use crate::Disk::*;

pub struct HeadInfo {
    blockType: i32,
    pblock: i32,
    lblock: i32,
    rblock: i32,
    numEntries: i32,
    numAttrs: i32,
    numSlots: i32,
    reserved: [u8; 4],
}

#[derive(Clone)]
pub enum Attribute {
    nVal(f64),
    sVal(String),
}

fn compare_attrs(attr1: Attribute, attr2: Attribute, attr_type: i32) -> i32 {
    // Implementation goes here
    unimplemented!()
}

#[derive(Clone)]
pub struct InternalEntry {
    pub lChild: i32,
    pub attrVal: Attribute,
    pub rChild: i32,
}

#[derive(Clone)]
pub struct Index {
    pub attrVal: Attribute,
    pub block: i32,
    pub slot: i32,
    pub unused: [u8; 8],
}

#[derive(Clone)]
pub struct BlockBuffer {
    blockNum: i32,
}

#[derive(Default, Clone, Copy)]
struct BufferMetaInfo {
    free: bool,
    dirty: bool,
    block_num: i32,
    time_stamp: i32,
}

lazy_static! {
    static ref BLOCKS: [[u8; BLOCK_SIZE]; BUFFER_CAPACITY] = [[0; BLOCK_SIZE]; BUFFER_CAPACITY];
    static ref METAINFO: [BufferMetaInfo; BUFFER_CAPACITY] =
        [BufferMetaInfo::default(); BUFFER_CAPACITY];
    static ref BLOCK_ALLOC_MAP: [u8; DISK_BLOCKS] = [0; DISK_BLOCKS];
}
impl BlockBuffer {
    fn loadBlockAndGetBufferPtr(&mut self) -> Result<&mut [u8], String> {
        // Implementation goes here
        unimplemented!()
    }

    fn getFreeBlock(&mut self, blockType: i32) -> Result<i32, String> {
        // Implementation goes here
        unimplemented!()
    }

    fn setBlockType(&mut self, blockType: i32) -> Result<(), String> {
        // Implementation goes here
        unimplemented!()
    }

    pub fn fromBlockType(blockType: char) -> Result<BlockBuffer, String> {
        // Implementation goes here
        unimplemented!()
    }

    pub fn fromBlockNum(blockNum: i32) -> Result<BlockBuffer, String> {
        // Implementation goes here
        Ok(BlockBuffer { blockNum })
    }

    pub fn getBlockNum(&self) -> i32 {
        self.blockNum
    }

    pub fn getHeader(&self, head: &mut HeadInfo) -> Result<(), String> {
        // Implementation goes here
        unimplemented!()
    }

    pub fn setHeader(&mut self, head: &HeadInfo) -> Result<(), String> {
        // Implementation goes here
        unimplemented!()
    }

    pub fn releaseBlock(&mut self) {
        // Implementation goes here
        unimplemented!()
    }
}

pub struct RecBuffer {
    blockBuffer: BlockBuffer,
}

impl RecBuffer {
    pub fn new() -> Result<RecBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockNum(0)?;
        Ok(RecBuffer { blockBuffer })
    }
    pub fn fromBlockNum(blockNum: i32) -> Result<RecBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockNum(blockNum)?;
        Ok(RecBuffer { blockBuffer })
    }

    pub fn getSlotMap(&self, slotMap: &mut [u8]) -> Result<(), String> {
        unimplemented!()
    }

    pub fn setSlotMap(&mut self, slotMap: &[u8]) -> Result<(), String> {
        unimplemented!()
    }

    pub fn getRecord(&self, rec: &mut Attribute, slotNum: i32) -> Result<(), String> {
        unimplemented!()
    }

    pub fn setRecord(&mut self, rec: &Attribute, slotNum: i32) -> Result<(), String> {
        unimplemented!()
    }

    /*
     * Rusty give no inherity :/
     */
    pub fn getBlockNum(&self) -> i32 {
        self.blockBuffer.getBlockNum()
    }
    pub fn getHeader(&self, head: &mut HeadInfo) -> Result<(), String> {
        self.blockBuffer.getHeader(head)
    }
    pub fn setHeader(&mut self, head: &HeadInfo) -> Result<(), String> {
        self.blockBuffer.setHeader(head)
    }
    pub fn releaseBlock(&mut self) {
        self.blockBuffer.releaseBlock()
    }
}

pub struct IndBuffer {
    blockBuffer: BlockBuffer,
}

impl IndBuffer {
    pub fn fromBlockNum(blockType: char) -> Result<IndBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockType(blockType)?;
        Ok(IndBuffer { blockBuffer })
        // Implementation goes here
    }
    pub fn fromBlockType(blockType: char) -> Result<IndBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockType(blockType)?;
        Ok(IndBuffer { blockBuffer })
        // Implementation goes here
    }

    pub fn getEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here
        unimplemented!()
    }

    pub fn setEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here
        unimplemented!()
    }
    /*
     * Rusty give no inherity :/
     */
    pub fn getBlockNum(&self) -> i32 {
        self.blockBuffer.getBlockNum()
    }
    pub fn getHeader(&self, head: &mut HeadInfo) -> Result<(), String> {
        self.blockBuffer.getHeader(head)
    }
    pub fn setHeader(&mut self, head: &HeadInfo) -> Result<(), String> {
        self.blockBuffer.setHeader(head)
    }
    pub fn releaseBlock(&mut self) {
        self.blockBuffer.releaseBlock()
    }
}

pub struct IndInternal {
    indBuffer: IndBuffer,
}

impl IndInternal {
    pub fn new() -> Result<RecBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockNum(0)?;
        Ok(RecBuffer { blockBuffer })
    }
    pub fn fromBlockNum(blockType: char) -> Result<IndBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockType(blockType)?;
        Ok(IndBuffer { blockBuffer })
        // Implementation goes here
    }
    pub fn getEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here (Overload or rewrite)
        unimplemented!()
    }

    pub fn setEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here (Overload or rewrite)
        unimplemented!()
    }
    /*
     * Rusty give no inherity :/
     */
    pub fn getBlockNum(&self) -> i32 {
        self.indBuffer.getBlockNum()
    }
    pub fn getHeader(&self, head: &mut HeadInfo) -> Result<(), String> {
        self.indBuffer.getHeader(head)
    }
    pub fn setHeader(&mut self, head: &HeadInfo) -> Result<(), String> {
        self.indBuffer.setHeader(head)
    }
    pub fn releaseBlock(&mut self) {
        self.indBuffer.releaseBlock()
    }
}

pub struct IndLeaf {
    indBuffer: IndBuffer,
}

impl IndLeaf {
    pub fn new() -> Result<RecBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockNum(0)?;
        Ok(RecBuffer { blockBuffer })
    }
    pub fn fromBlockNum(blockType: char) -> Result<IndBuffer, String> {
        let blockBuffer = BlockBuffer::fromBlockType(blockType)?;
        Ok(IndBuffer { blockBuffer })
        // Implementation goes here
    }
    pub fn getEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here (Overload or rewrite)
        unimplemented!()
    }

    pub fn setEntry(&self, ptr: *mut u8, indexNum: i32) -> i32 {
        // Implementation goes here (Overload or rewrite)
        unimplemented!()
    }
    /*
     * Rusty give no inherity :/
     */
    pub fn getBlockNum(&self) -> i32 {
        self.indBuffer.getBlockNum()
    }
    pub fn getHeader(&self, head: &mut HeadInfo) -> Result<(), String> {
        self.indBuffer.getHeader(head)
    }
    pub fn setHeader(&mut self, head: &HeadInfo) -> Result<(), String> {
        self.indBuffer.setHeader(head)
    }
    pub fn releaseBlock(&mut self) {
        self.indBuffer.releaseBlock()
    }
}
fn getFreeBuffer(blockNum: i32) -> i32 {
    unimplemented!()
}

fn getBufferNum(blockNum: i32) -> i32 {
    unimplemented!()
}

pub fn getStaticBlockType(blockNum: i32) -> i32 {
    unimplemented!()
}

pub fn setDirtyBit(blockNum: i32) -> i32 {
    unimplemented!()
}
