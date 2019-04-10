pub mod product;

use array::Array;
use shape::Shape;

/// Checks if array is matrix like
///
/// # Arguments
///
/// * `array` - array
#[inline]
fn is_matrix<T: Clone>(array: &Array<T>) -> bool {
    return array.get_shape().len() == 2;
}

/// Checks if array is vector like
///
/// # Arguments
///
/// * `array` - array
#[inline]
fn is_vector<T: Clone>(array: &Array<T>) -> bool {
    return array.get_shape().len() == 1;
}
