mod arithmetic;
mod relational;

use shape::Shape;


/// Checks if shapes are compatible - have same dimensions and length or second one is unit array
///
/// # Arguments
///
/// * `first` - the first shape
/// * `second` - the second shape
#[inline]
fn check_shapes_compatibility(first: &Shape, second: &Shape) -> () {
    if first != second && second.total_len() != 1 {
        panic!("Shapes are not compatible");
    }
}
