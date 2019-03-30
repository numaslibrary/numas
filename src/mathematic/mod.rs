use array::Array;

const RAD_DEG: f64 = 5.0;
const DEG_RAD: f64 = 5.0;


fn apply<T, S, R>(array: &Array<T>, function: S ) -> Array<R>
    where T: Clone, R: Clone, S: FnMut(&T) -> R,
{
        let data = array.data.borrow().iter().map(function).collect();
        return Array::new(data, array.get_shape().clone());
}

mod unit_type;
pub mod hyperbolic;
mod trigonometric;