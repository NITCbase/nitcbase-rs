pub mod Disk {
    use crate::define::*;
    use std::fs::{File, OpenOptions};
    use std::io::{self, Error, ErrorKind, Read, Seek, Write};
    pub fn new_disk() -> io::Result<()> {
        // Copy Disk to Disk Run Copy
        let mut src_file = File::open(DISK_PATH)?;
        let mut dst_file = File::create(DISK_RUN_COPY_PATH)?;

        io::copy(&mut src_file, &mut dst_file)?;

        Ok(())
    }
    pub fn drop_disk() {
        // Copy Disk Run Copy to Disk
        if let Err(e) = std::fs::copy(DISK_RUN_COPY_PATH, DISK_PATH) {
            println!("Error updating disk: {}", e);
        }
    }

    pub fn read_block(block: &mut [u8], block_num: usize) -> io::Result<()> {
        if block_num >= DISK_BLOCKS {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Block number out of bounds",
            ));
        }

        let offset = block_num * BLOCK_SIZE;
        let mut file = File::open(DISK_RUN_COPY_PATH)?;
        file.seek(io::SeekFrom::Start(offset as u64))?;
        file.read_exact(block)?;

        Ok(())
    }

    pub fn write_block(block: &[u8], block_num: usize) -> io::Result<()> {
        if block_num >= DISK_BLOCKS {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Block number out of bounds",
            ));
        }

        let offset = block_num * BLOCK_SIZE;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(DISK_RUN_COPY_PATH)?;
        file.seek(io::SeekFrom::Start(offset as u64))?;
        file.write_all(block)?;

        Ok(())
    }
}
pub use self::Disk::*;
