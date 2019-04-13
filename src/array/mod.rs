mod array;
mod slice;

pub use self::array::Array;

/// Creates unit array (array with only one element)
///
/// # Examples
///
/// ```
///  #[macro_use] extern crate numas;
///  use numas::array::Array;
///
///  let array = u![8];
///  let data = array.collect();
///
///  assert_eq!(data, vec![8]);
///  assert_eq!(array.len(), 1);
/// ```
#[macro_export]
macro_rules! u {
    ($y:expr) => {
        Array::new(vec![$y], vec![1])
    }
}

