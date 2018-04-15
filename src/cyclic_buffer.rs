use managed::ManagedSlice;

pub struct CyclicBuff<'a, T: 'a> {
    storage: ManagedSlice<'a, T>,
    length: usize,
    start: usize

}

impl<'a, T: 'a> CyclicBuff<'a, T: 'a> {
    pub fn new<S>(storage: S) -> CyclicBuff<'a, T> 
        where S: Into<ManagedSlice<'a, T>>
    {
        CyclicBuff {
            storage: storage.into(),
            length: 0,
            start: 0,
        }
    }

    pub fn push<T>(&self, data: T) -> Result<R> {
        let mut next: usize = self.start + 1;
        if next > length {
            next = 0;
        }

        if next == 
    }


}