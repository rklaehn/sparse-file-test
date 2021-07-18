use std::{fs::OpenOptions, os::unix::prelude::{AsRawFd}};

use hole_punch::SparseFile;
use memmap::MmapOptions;
use nix::fcntl::FallocateFlags;

fn main() -> anyhow::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("sparse-test")?;
    let fd = file.as_raw_fd();
    nix::fcntl::fallocate(fd, FallocateFlags::empty(), 0, 1024 * 1024 * 1024 * 4)?;
    let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };
    println!("write 4g");
    mmap.fill(1u8);
    mmap.flush()?;
    println!("punch 100 holes");
    for i in 0..100 {
        nix::fcntl::fallocate(fd, FallocateFlags::FALLOC_FL_KEEP_SIZE | FallocateFlags::FALLOC_FL_PUNCH_HOLE, i * 20000, 10000)?;
    }
    println!("showing holes");
    let chunks = file.scan_chunks()?;
    for segment in chunks {
        println!("{:#x}..{:#x}", segment.start, segment.end);
    }
    Ok(())
}
