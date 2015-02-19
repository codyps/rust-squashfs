
pub trait WriteAt {
    // TODO: remove mut from self & use native calls
    pub fn write_at(&mut self, &[u8], u64) -> io::Result<()>;
};

pub trait ReadAt {
    // TODO: remove mut from self & use native calls
    pub fn read_at(&mut self, &mut [u8], u64) -> io::Result<usize>;
}

// FIXME: need negative bounds or (even better) a "best matching" approach like C++
impl ReadAt for Read+Seek {
    pub fn read_at(&mut self, data: &mut [u8], offs: u64) {
        try!(self.seek(io::Seek
    }
}

