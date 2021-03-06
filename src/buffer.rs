use ffi::TF_Buffer;
use libc::size_t;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use Result;
use memory::Memory;

/// A buffer.
pub struct Buffer {
    memory: Memory<u8>,
    raw: TF_Buffer,
}

impl Buffer {
    /// Create a buffer.
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Memory::new(data).into()
    }

    /// Create a buffer from raw parts.
    #[inline]
    pub unsafe fn from_raw_parts(pointer: *mut u8, length: usize) -> Buffer {
        Memory::from_raw_parts(pointer, length).into()
    }

    /// Load a buffer.
    pub fn load<T>(path: T) -> Result<Self> where T: AsRef<Path> {
        let mut data = vec![];
        let mut file = ok!(File::open(path));
        ok!(file.read_to_end(&mut data));
        Ok(Buffer::new(data))
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_raw(&self) -> *mut TF_Buffer {
        &self.raw as *const _ as *mut _
    }

    #[doc(hidden)]
    pub unsafe fn reset(&mut self) {
        self.memory = Memory::from_raw_parts(self.raw.data as *mut _, self.raw.length as usize);
    }
}

memory!(Buffer<u8>);

#[doc(hidden)]
impl From<Memory<u8>> for Buffer {
    fn from(memory: Memory<u8>) -> Self {
        let raw = TF_Buffer {
            data: memory.as_ptr() as *mut _,
            length: memory.len() as size_t,
            data_deallocator: None,
        };
        Buffer { memory: memory, raw: raw }
    }
}
