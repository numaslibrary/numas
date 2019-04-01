use array::Array;
use shape::Shape;


/// Creates new array of given shape filled with given value
///
/// # Arguments
///
/// * `value` - value to fill array with
/// * `shape` - shape of new array
pub fn fill<T: Clone>(value: T, shape: Vec<i32>) -> Array<T> {
    let len: i32 = shape.iter().product();
    return Array::new(vec![value; len as usize], shape);
}

/// Creates new array of given shape filled with zeros
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn zeros<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return fill::<T>(T::from(0), shape);
}

/// Creates new array of given shape filled with zeros
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn zeroes<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return zeros::<T>(shape);
}

/// Creates new array of given shape filled with ones
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn ones<T: Clone + From<u8>>(shape: Vec<i32>) -> Array<T> {
    return fill::<T>(T::from(1), shape);
}