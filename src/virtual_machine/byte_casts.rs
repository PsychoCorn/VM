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


pub fn from_bytes<T>(bytes: &[u8]) -> Option<&[T]> {
    let bytes_counter = bytes.len();
    if bytes_counter % size_of::<T>() != 0 {
        None
    } else {
        let data = 
            bytes.as_ptr() as *const T;
        Some (
            unsafe {
                &* slice_from_raw_parts(
                    data, 
                    bytes_counter / size_of::<T>()
                )
            }
        )
    }
}