pub mod product;

use array::Array;

/// Checks if array is matrix like
///
/// # Arguments
///
/// * `array` - array
///
/// # Examples
///
/// ```
/// use numas::array::Array;
///
/// let matrix = Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]);
/// let vector = Array::new(vec![1, 2, 3, 4, 5, 6], vec![6]);
///
/// assert_eq!(numas::linear_algebra::is_matrix(&matrix), true);
/// assert_ne!(numas::linear_algebra::is_matrix(&vector), true);
/// ```
#[inline]
pub fn is_matrix<T: Clone>(array: &Array<T>) -> bool {
    return array.get_shape().len() == 2;
}

/// Checks if array is vector like
///
/// # Arguments
///
/// * `array` - array
///
/// # Examples
///
/// ```
/// use numas::array::Array;
///
/// let matrix = Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]);
/// let vector = Array::new(vec![1, 2, 3, 4, 5, 6], vec![6]);
///
/// assert_eq!(numas::linear_algebra::is_vector(&vector), true);
/// assert_ne!(numas::linear_algebra::is_vector(&matrix), true);
/// ```
#[inline]
pub fn is_vector<T: Clone>(array: &Array<T>) -> bool {
    return array.get_shape().len() == 1;
}
