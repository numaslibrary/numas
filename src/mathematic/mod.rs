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
        let old_data = array.data.borrow();
        let data: Vec<R> = old_data[array.shape().get_bounds()].iter().map(function).collect();

        return Array::new(data, array.get_shape().clone());
}

mod hyperbolic;
mod trigonometric;
mod reducer;
mod miscellaneous;
mod round;
mod logarithm;
mod exponential;
