use array::Array;
use std::ops::{Add, Mul};


/// Returns the dot product of two vector like arrays
///
/// # Arguments
///
/// * `first` - first vector like array
/// * `second` - second vector like array
pub fn vdot<T>(first: &Array<T>, second: &Array<T>) -> T
    where T: Clone + Add<Output=T> + Mul<Output=T> + From<u8>
{
    if !super::is_vector(&first) || !super::is_vector(&second) {
        panic!("Calling method vdot with non vector like arrays is not supported");
    }

    let first_data = first.data.borrow();
    let second_data = second.data.borrow();

    return first.shape().get_bounds()
        .zip(second.shape().get_bounds())
        .fold(T::from(0), |acc, (f, s)| acc + first_data[f].clone() * second_data[s].clone());
}
