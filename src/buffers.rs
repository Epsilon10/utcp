pub struct uTcpBuffer<'a, T: 'a> {
    storage: ManagedSlice<'a, T>,
    length: usize
}

pub struct PacketMetaData<H> {
    size: usize,
    header: Option<H>
}

pub struct PacketBuffer<'a, 'b, H: 'a> {
    metadata: CyclicBuff<'a, PacketMetaData<H>>,
    payload: CyclicBuff<'b, u8>
}

impl<'a, T: 'a> PacketBuffer<'a, T> {
    pub fn new<S>(storage: S) -> TcpBuffer<'a, T> 
        where S:Into<ManagedSlice<'a, T>>
    {
        RingBuffer {
            storage: storage.into(),
            length: 0
        }
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    pub fn window(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.window == 0
    }

    pub fn enqueue(&mut self, size: usize, header: H) -> Result<&mut [u8]> {
        if self.payload.capacity() < size {
            return Err("bleh")
        }

        if self.
    }


}