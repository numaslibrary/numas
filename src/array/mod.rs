mod array;
mod slice;

pub use self::array::Array;

/// Creates unit array (array with only one element)
#[macro_export]
macro_rules! u {
    ($y:expr) => {
        Array::new(vec![$y], vec![1])
    }
}

