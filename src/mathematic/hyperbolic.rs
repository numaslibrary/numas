use array::Array;


/// Applies hyperbolic sine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn sinh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::sinh(value.clone().into()));
}

/// Applies hyperbolic cosine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn cosh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::cosh(value.clone().into()));
}

/// Applies hyperbolic tangent on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn tanh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::tanh(value.clone().into()));
}

/// Applies inverse hyperbolic sine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arcsinh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::asinh(value.clone().into()));
}

/// Applies hyperbolic cosine on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arccosh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::acosh(value.clone().into()));
}

/// Applies hyperbolic tangent on elements from given array and creates new array
///
/// # Arguments
///
/// * `array` - source array
#[inline]
pub fn arctanh<T>(array: &Array<T>) -> Array<f64>
    where T: Clone + Into<f64>
{
    return super::apply(array, |value: &T| f64::atanh(value.clone().into()));
}
