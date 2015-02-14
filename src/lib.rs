#![feature(io)]

extern crate "rustc-serialize" as rustc_serialize;

use std::io::{Read, Seek};

#[derive(RustcDecodable, RustcEncodable)]
struct Super {
    s_magic: u32,
    inodes: u32,
    mkfs_time: u32,
    block_size: u32,
    fragments: u32,
    compression: u16,
    block_log: u16,
    flags: u16,
    no_ids: u16,
    s_major: u16,
    s_minor: u16,
    root_inode: u64,
    bytes_used: u64,
    id_table_start: u64,
    xattr_id_table_start: u64,
    inode_table_start: u64,
    directory_table_start: u64,
    fragment_table_start: u64,
    lookup_table_start: u64,
}

pub struct File<'a, R: Read + Seek + 'a> {
    /* FIXME: mutability is only for Seek+Reader's limitation in needing a cursor.
     * Once we have it, change to ReaderAt and use a non-mut reference
     */
    a: &'a mut R
}

impl<'b, R: Read+Seek + 'b> File<'b, R> {
    fn new(r: &'b mut R) -> File<'b, R>
    {
        File { a : r }
    }

    fn size(&mut self) -> u64
    {

    }
}

#[test]
fn file_new() {
    use std::fs;
    let mut io = fs::File::open("tmp/empty.squashfs").unwrap();
    let f = File::new(&mut io);
}
