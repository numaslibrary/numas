use array::Array;
use shape::Shape;


/// Creates new array of given shape filled with given value
///
/// # Arguments
///
/// * `value` - value to fill array with
/// * `shape` - shape of new array
///
/// # Examples
///
/// ```
///  use numas::array::Array;
///  use numas::shape::Shape;
///
///  let array = numas::factory::fill::full::<i32>(3, vec![2, 2]);
///  let data = array.collect();
///
///  assert_eq!(data, vec![3; 4]);
/// ```
pub fn full<T: Clone>(value: T, shape: Vec<i32>) -> Array<T> {
    let len: i32 = shape.iter().product();
    return Array::new(vec![value; len as usize], shape);
}

/// Creates new array of given shape filled with zeros
///
/// # Arguments
///
/// * `shape` - shape of new array
///
/// # Examples
///
/// ```
///  use numas::array::Array;
///  use numas::shape::Shape;
///
///  let array = numas::factory::fill::zeros::<f64>(vec![2, 2]);
///  let data = array.collect();
///
///  assert_eq!(data, vec![0.0 as f64; 4]);
/// ```
#[inline]
pub fn zeros<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return full::<T>(T::from(0), shape);
}

/// Creates new array of given shape filled with zeros
///
/// # Arguments
///
/// * `shape` - shape of new array
///
/// # Examples
///
/// ```
///  use numas::array::Array;
///  use numas::shape::Shape;
///
///  let array = numas::factory::fill::zeroes::<f64>(vec![2, 2]);
///  let data = array.collect();
///
///  assert_eq!(data, vec![0.0 as f64; 4]);
/// ```
#[inline]
pub fn zeroes<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return zeros::<T>(shape);
}

/// Creates new array of given shape filled with ones
///
/// # Arguments
///
/// * `shape` - shape of new array
///
/// # Examples
///
/// ```
///  use numas::array::Array;
///  use numas::shape::Shape;
///
///  let array = numas::factory::fill::ones::<i32>(vec![2, 2]);
///  let data = array.collect();
///
///  assert_eq!(data, vec![1 as i32; 4]);
/// ```
#[inline]
pub fn ones<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return full::<T>(T::from(1), shape);
}
