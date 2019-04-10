use array::Array;


/// Applies given function on given array elements and returns new array
///
/// # Arguments
///
/// * `array` - source array
/// * `function` - function to apply
fn apply<T, S, R>(array: &Array<T>, function: S ) -> Array<R>
    where T: Clone, R: Clone, S: FnMut(&T) -> R,
{
        let data = array.data.borrow().iter().map(function).collect();
        return Array::new(data, array.get_shape().clone());
}

pub mod hyperbolic;
pub mod trigonometric;
pub mod reducer;
