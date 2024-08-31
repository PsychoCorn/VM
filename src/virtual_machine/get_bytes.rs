use std::ptr::slice_from_raw_parts;
use super::Word;

pub trait GetBytes
    where
        Self : Copy 
{
    fn get_bytes(&self) -> &[u8] {
        let data = 
            self as *const Self
                 as *const u8;

        unsafe {
            &* slice_from_raw_parts(
                data, 
                size_of::<Self>()
            )
        }
    }
}

impl GetBytes for Word {}

impl<T: Copy> GetBytes for &[T] {
    fn get_bytes(&self) -> &[u8] {
        let len = self.len() * size_of::<T>();
        let data = self.as_ptr() as *const u8;

        unsafe {
            &* slice_from_raw_parts(data, len)
        }
    }
}