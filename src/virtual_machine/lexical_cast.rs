use super::{Word, SWord, Real};

/// cast from Self to T without changing bit representation
pub trait LexicalCast 
    where 
        Self: Copy 
{   
    fn lexical_cast<T: Copy>(&self) -> Option<T> {
        if size_of::<Self>() != size_of::<T>() ||
           align_of::<Self>() != align_of::<T>() 
        {
            None
        } else {
            Some(
                unsafe{
                    * (
                        self
                            as *const Self
                            as *const T
                    )
                }
            )
        }
    }
}

macro_rules! impl_lexical_cast {
    ( $( $T:ty ), * ) => {
        $(

            impl LexicalCast for $T {}

        )*
    };
}

impl_lexical_cast!(Word, SWord, Real);

