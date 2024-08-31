use super::Word;

pub trait IntoChar {
    fn into_char(&self) -> Option<char>;
}

impl IntoChar for Word {
    fn into_char(&self) -> Option<char> {
        if *self > u32::MAX as Self {
            None
        } else {
            char::from_u32(*self as u32)
        }
    }
}