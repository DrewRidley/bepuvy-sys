/// A raw byte buffer.
#[repr(C)]
pub struct ByteBuffer {
    /// Pointer to the beginning of the memory backing this buffer.
    pub memory: *mut u8,
    /// Length of the buffer in bytes.
    pub len: i32,
    /// Implementation specific identifier of the raw buffer set by its source. If taken from a BufferPool, Id includes the index in the power pool from which it was taken.
    pub id: i32,
}

impl ByteBuffer {
    /// Unsafely casts the byte buffer to a typed buffer.
    pub unsafe fn as_buffer<T>(&self) -> Buffer<T> {
        Buffer {
            memory: self.memory as *mut T,
            len: self.len / std::mem::size_of::<T>() as i32,
            id: self.id,
        }
    }
}

/// Span over an unmanaged memory region.
#[repr(C)]
pub struct Buffer<T> {
    /// Pointer to the beginning of the memory backing this buffer.
    pub memory: *mut T,
    /// Length of the buffer in typed elements.
    pub len: i32,
    /// Implementation specific identifier of the raw buffer set by its source. If taken from a BufferPool, Id includes the index in the power pool from which it was taken.
    pub id: i32,
}

impl<T> std::ops::Index<i32> for Buffer<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        assert!(index >= 0 && index < self.len);
        unsafe { &*self.memory.offset(index as isize) }
    }
}

impl<T> std::ops::IndexMut<i32> for Buffer<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        assert!(index >= 0 && index < self.len);
        unsafe { &mut *self.memory.offset(index as isize) }
    }
}

impl<T> From<ByteBuffer> for Buffer<T> {
    fn from(buffer: ByteBuffer) -> Self {
        Self {
            memory: buffer.memory as *mut T,
            len: buffer.len / std::mem::size_of::<T>() as i32,
            id: buffer.id,
        }
    }
}

impl<T> From<&ByteBuffer> for Buffer<T> {
    fn from(buffer: &ByteBuffer) -> Self {
        Self {
            memory: buffer.memory as *mut T,
            len: buffer.len / std::mem::size_of::<T>() as i32,
            id: buffer.id,
        }
    }
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self {
            memory: std::ptr::null_mut(),
            len: 0,
            id: 0,
        }
    }

    pub fn from_raw(memory: *mut T, length: i32, id: i32) -> Self {
        Self {
            memory,
            len: length,
            id,
        }
    }
}

impl<T> From<Buffer<T>> for ByteBuffer {
    fn from(buffer: Buffer<T>) -> Self {
        Self {
            memory: buffer.memory as *mut u8,
            len: buffer.len * std::mem::size_of::<T>() as i32,
            id: buffer.id,
        }
    }
}

impl std::ops::Index<i32> for ByteBuffer {
    type Output = u8;

    fn index(&self, index: i32) -> &Self::Output {
        assert!(index >= 0 && index < self.len);
        unsafe { &*self.memory.offset(index as isize) }
    }
}

impl std::ops::IndexMut<i32> for ByteBuffer {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        assert!(index >= 0 && index < self.len);
        unsafe { &mut *self.memory.offset(index as isize) }
    }
}

/// A quick list that avoids heap allocations for growing.
#[repr(C)]
pub struct QuickList<T> {
    /// Backing memory containing the elements of the list.
    /// Indices from 0 to Count-1 hold actual data. All other data is undefined.
    pub span: Buffer<T>,
    /// Number of elements in the list.
    pub count: i32,
}

impl<T> std::ops::Index<i32> for QuickList<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        assert!(index >= 0 && index < self.count);
        &self.span[index]
    }
}

impl<T> std::ops::IndexMut<i32> for QuickList<T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        assert!(index >= 0 && index < self.count);
        &mut self.span[index]
    }
}

impl<T> QuickList<T> {
    pub fn new() -> Self {
        Self {
            span: Buffer::new(),
            count: 0,
        }
    }
}
