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
pub fn zeros(shape: Vec<i32>) -> Array<i32> {
    return fill(0, shape);
}

/// Creates new array of given shape filled with zeros
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn zeroes(shape: Vec<i32>) -> Array<i32> {
    return zeros(shape);
}

/// Creates new array of given shape filled with ones
///
/// # Arguments
///
/// * `shape` - shape of new array
#[inline]
pub fn ones(shape: Vec<i32>) -> Array<i32> {
    return fill(1, shape);
}