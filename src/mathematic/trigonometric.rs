use array::Array;


/// Applies sine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn sin<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::sin(value.clone().into()));
}

/// Applies cosine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn cos<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::cos(value.clone().into()));
}

/// Applies inverse sine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arcsin<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::asin(value.clone().into()));
}

/// Applies inverse cosine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arccos<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::acos(value.clone().into()));
}

/// Applies inverse tangent on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arctan<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::atan(value.clone().into()));
}

/// Converts elements from given array to degrees and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn degrees<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::to_degrees(value.clone().into()));
}

/// Converts elements from given array to radians and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn radians<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::to_radians(value.clone().into()));
}

/// Converts elements from given array to degrees and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn deg2rad<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return radians(array);
}

/// Converts elements from given array to radians and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn rad2deg<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return degrees(array);
}
