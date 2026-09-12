use std::mem::zeroed;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TOptional<T: Clone>
{
    pub value: T,
    pub is_set: bool,
}

impl<T: Clone> TOptional<T>
{
    pub const fn none() -> TOptional<T>
    {
        return TOptional { value: unsafe { zeroed() }, is_set: false }

    }
    pub fn some(data: T) -> TOptional<T>
    {
        return TOptional { value: data, is_set: true }
    }
    pub fn reset(&mut self)
    {
        self.is_set = false;
    }
    pub fn set(&mut self, data: T)
    {
        self.value = data;
        self.is_set = true;
    }
    pub fn get(&self) -> T
    {
        assert!(self.is_set);
        return self.value.clone();
    }
}
